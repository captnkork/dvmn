use num_traits::Num;

#[derive(Debug)]
struct Range<T: Num + PartialOrd + Copy + std::fmt::Display> {
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
struct Monitor {
    current: usize,
    previous: usize,
    samples: Range<usize>,
    values: Range<f64>,
    nominal: Range<f64>,
}

#[derive(Debug)]
enum Deviation {
    Low(f64),
    High(f64),
}

#[derive(Debug)]
enum State {
    Nominal(f64),
    Alert(f64, Deviation),
    Error(f64, Deviation),
}

impl Monitor {
    fn new(
        samples: Range<usize>,
        values: Range<f64>,
        nominal: Range<f64>,
    ) -> Result<Monitor, String> {
        match &values.contains(&nominal) {
            true => Ok(Monitor {
                current: 0,
                previous: 0,
                samples,
                values,
                nominal,
            }),
            false => Err(format!(
                "values nominal:={:?} is not contained in values:={:?}",
                &nominal, &values
            )),
        }
    }

    fn update(mut self, sample: usize) {
        self.previous = self.current;
        self.current = sample;
    }

    fn state(self) -> State {
        let size = self.values.size();

        State::Nominal(0.0)
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
        assert_eq!(0, monitor.current);
        assert_eq!(0, monitor.previous);
        assert_eq!(0x00, monitor.samples.min);
        assert_eq!(0xff, monitor.samples.max);
        assert_eq!(0.0, monitor.values.min);
        assert_eq!(1.0, monitor.values.max);
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
