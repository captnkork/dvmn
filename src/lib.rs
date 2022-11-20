use num_traits::Num;
use std::time::{self, Instant};


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

#[derive(Debug)]
struct Range {
    min: f64,
    max: f64,
}

impl Range {
    fn new(min: f64, max: f64) -> Result<Range, String> {
        match min <= max {
            true => Ok(Range { min, max }),
            false => Err(format!(
                "invalid range: min:={}, max:={}, min <= max is false!",
                min, max
            )),
        }
    }

    fn contains(&self, range: &Range) -> bool {
        &self.min <= &range.min && &range.max <= &self.max
    }

    fn deviation(&self, x: f64) -> Option<f64> {
        if x < self.min {
            return Some(x - self.min) 
        }
        if x > self.max {
            return Some(x - self.max) 
        }
        None
    }

    fn size(&self) -> f64 {
        self.max - self.min
    }
}

#[derive(Debug)]
struct Sample {
    value: f64,
    time: time::Instant,
}

impl Sample {
    fn new(x: usize) -> Sample {
        Sample {
            value: x as f64,
            time: time::Instant::now(),
        }
    }
}

#[derive(Debug)]
struct Monitor {
    domain: Range,
    destination: Range,
    nominal: Range,
}

impl Monitor {
    fn new(
        domain: Range,
        destination: Range,
        nominal: Range,
    ) -> Result<Monitor, String> {
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

    fn f(& self, x: f64) -> f64 {
        self.destination.min + ( x * ( self.destination.size() / self.domain.size() ) )
    }

    fn validate(&mut self, current: Sample, previous: Sample) -> State {
        
        let current_value = self.f(current.value);
        let previous_value = self.f(previous.value);

        //if current

        State::Nominal(current_value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_range() {
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
    fn range_deviation() {
        for off in [-1.0, -0.5, 0.0, 0.5, 1.0] {
            let range = Range::new(off - 1.0, off + 1.0).unwrap();

            assert_eq!(None, range.deviation(off - 1.0));
            assert_eq!(None, range.deviation(off));
            assert_eq!(None, range.deviation(off + 1.0));
            assert!( (range.deviation(off - 1.1).unwrap() - (-0.1)).abs() < 1e-10, " at off = {}", off);
            assert!( (range.deviation(off + 1.1).unwrap() - (0.1)).abs() < 1e-10, "at off = {}", off);
        }
    }
    #[test]
    fn new_monitor() {
        let monitor = Monitor::new(
            Range::new(0.0, 256.0).unwrap(),
            Range::new(0.0, 1.0).unwrap(),
            Range::new(0.2, 0.7).unwrap(),
        )
        .unwrap();
        assert_eq!(0.0, monitor.domain.min);
        assert_eq!(256.0, monitor.domain.max);
        assert_eq!(0.0, monitor.destination.min);
        assert_eq!(1.0, monitor.destination.max);
        assert_eq!(0.2, monitor.nominal.min);
        assert_eq!(0.7, monitor.nominal.max);
    }
    #[test]
    fn new_monitor_invalid_spec() {
        let result = Monitor::new(
            Range::new(0.0, 256.0).unwrap(),
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
