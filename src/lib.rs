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
struct Monitor<TDomain, TDestination> {
    domain: Range<TDomain>,
    destination: Range<TDestination>,
    nominal: Range<TDestination>,
}

impl<TDomain, TDestination> Monitor<TDomain, TDestination>
where
    TDomain: Num + PartialOrd + Copy + std::fmt::Display + std::fmt::Debug,
    TDestination: Num + PartialOrd + Copy + std::fmt::Display + std::fmt::Debug,
{
    fn new(
        domain: Range<TDomain>,
        destination: Range<TDestination>,
        nominal: Range<TDestination>,
    ) -> Result<Monitor<TDomain, TDestination>, String> {
        match &destination.contains(&nominal) {
            true => Ok(Monitor {
                domain,
                destination,
                nominal,
            }),
            false => Err(format!(
                "values nominal:={:?} is not contained in values:={:?}",
                &nominal, &destination
            )),
        }
    }

    fn state(
        &mut self,
        current: Sample<TDomain>,
        previous: Sample<TDomain>,
    ) -> State<TDestination> {
        let size = self.domain.size();

        State::Nominal(TDestination::zero())
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
