use num_traits::Num;
use std::time;

#[derive(Debug)]
struct Range<T> {
    min: T,
    max: T,
}

impl<T: Num + PartialOrd + Copy + std::fmt::Display> Range<T> {
    fn new(min: T, max: T) -> Result<Range<T>, String> {
        match min <= max {
            true => Ok(Range { min, max }),
            false => Err(format!(
                "invalid range: min:={}, max:={}, min <= max is false!",
                min, max
            )),
        }
    }

    fn contains(&self, range: &Range<T>) -> bool {
        &self.min <= &range.min && &range.max <= &self.max
    }

    fn size(&self) -> T {
        self.max - self.min
    }
}

#[derive(Debug)]
struct Sample<T: Num + PartialOrd + Copy + std::fmt::Display> {
    value: T,
    time: time::Instant,
}

impl<T: Num + PartialOrd + Copy + std::fmt::Display> Sample<T> {
    fn new(value: T) -> Sample<T> {
        Sample {
            value,
            time: time::Instant::now(),
        }
    }
}

#[derive(Debug)]
enum Deviation<T> {
    Low(T),
    High(T),
}

#[derive(Debug)]
enum State<T: Num> {
    Nominal(T),
    Alert(T, Deviation<T>),
    Error(T, Deviation<T>),
}

#[derive(Debug)]
struct Monitor<T> {
    domain: Range<T>,
    destination: Range<T>,
    nominal: Range<T>,
}

impl<T: Num + PartialOrd + Copy + std::fmt::Display + std::fmt::Debug> Monitor<T> {
    fn new(
        domain: Range<T>,
        destination: Range<T>,
        nominal: Range<T>,
    ) -> Result<Monitor<T>, String> {
        if let false = &destination.contains(&nominal) {
            return Err(format!(
                "values nominal:={:?} is not contained in values:={:?}",
                &nominal, &destination
            ));
        }

        Ok(Monitor {
            domain,
            destination,
            nominal,
        })
    }

    fn validate(&mut self, current: Sample<T>, previous: Sample<T>) -> State<T> {
        let value = self.domain.size();

        State::Nominal(T::zero())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_range() {
        {
            let range = Range::new(0x00, 0xff).unwrap();
            assert_eq!(0x00, range.min);
            assert_eq!(0xff, range.max);
        }

        {
            let range = Range::new(0.5, 0.6).unwrap();
            assert_eq!(0.5, range.min);
            assert_eq!(0.6, range.max);
        }

        {
            let range = Range::new(0.7, 0.7).unwrap();
            assert_eq!(0.7, range.min);
            assert_eq!(0.7, range.max);
        }
    }

    #[test]
    fn new_range_invalid_spec() {
        let result = Range::new(0.2, 0.1);
        match result {
            Err(str) => assert_eq!(
                "invalid range: min:=0.2, max:=0.1, min <= max is false!",
                str
            ),
            Ok(_) => assert!(false),
        }
    }

    #[test]
    fn range_contains() {
        assert!(Range::new(0.0, 1.0)
            .unwrap()
            .contains(&Range::new(0.1, 0.9).unwrap()));
        assert!(Range::new(0.1, 0.9)
            .unwrap()
            .contains(&Range::new(0.1, 0.9).unwrap()));
        assert!(!Range::new(0.1, 0.9)
            .unwrap()
            .contains(&Range::new(0.0, 1.0).unwrap()));
        assert!(!Range::new(0.1, 0.9)
            .unwrap()
            .contains(&Range::new(0.1, 1.0).unwrap()));
        assert!(!Range::new(0.1, 1.0)
            .unwrap()
            .contains(&Range::new(0.0, 1.0).unwrap()));
    }
    #[test]
    fn new_monitor() {
        let monitor = Monitor::new(
            Range::new(0x00, 0xff).unwrap(),
            Range::new(0.0, 1.0).unwrap(),
            Range::new(0.2, 0.7).unwrap(),
        )
        .unwrap();
        assert_eq!(0x00, monitor.domain.min);
        assert_eq!(0xff, monitor.domain.max);
        assert_eq!(0.0, monitor.destination.min);
        assert_eq!(1.0, monitor.destination.max);
        assert_eq!(0.2, monitor.nominal.min);
        assert_eq!(0.7, monitor.nominal.max);
    }
    #[test]
    fn new_monitor_invalid_spec() {
        let result = Monitor::new(
            Range::new(0x00, 0xff).unwrap(),
            Range::new(0.2, 1.0).unwrap(),
            Range::new(0.1, 0.7).unwrap(),
        );
        match result {
            Err(str) => assert_eq!(
                "values nominal:=Range { min: 0.1, max: 0.7 } is not contained in values:=Range { min: 0.2, max: 1.0 }",
                str
            ),
            Ok(_) => assert!(false),
        }
    }
}
