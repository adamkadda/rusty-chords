use super::{interval::Interval, third::Third, triad::Triad};

#[derive(Debug, PartialEq)]
pub struct Chord {
    pub third: Option<Third>,
    fifth: bool,
    seventh: bool,
    ninth: bool,
    eleventh: bool,
    triad: Option<Triad>,
    pub sus: Vec<Interval>,
    pub add: Vec<Interval>,
}

#[allow(dead_code)]
impl Chord {
    pub fn new() -> Chord {
        Chord {
            third: None,
            fifth: false,
            seventh: false,
            ninth: false,
            eleventh: false,
            triad: None,
            sus: Vec::new(),
            add: Vec::new(),
        }
    }


    fn handle_third(&mut self, interval: Interval) {
        if interval == Interval::Min3 {
            self.third = Some(Third::Minor);
            match self.sus.pop() {
                None => (),
                Some(value) => self.add.push(value),
            }
        } else if interval == Interval::Maj3 {
            if let Some(Third::Minor) = self.third {
                self.add.push(Interval::Min3)
            }
            self.third = Some(Third::Major);
            match self.sus.pop() {
                None => (),
                Some(value) => self.add.push(value),
            }
        } else { // Min2 | Maj2 | Perf4 |
            if let None = self.third {
                self.sus.push(interval);
            } else {
                self.add.push(interval);
            }
        }
    }


    fn handle_fifth(&mut self, interval: Interval) -> Result<(), &str>{
        match interval {
            Interval::Aug4 | Interval::Dim5 => {
                match &self.third {
                    None => self.sus.push(interval),
                    Some(third) => {
                        match third {
                            Third::Minor => self.triad = Some(Triad::Diminished),
                            Third::Major => self.add.push(interval),
                        }
                    }
                }
            }
            Interval::Perf5 => {
                self.fifth = true;
                match &self.third {
                    None => (),
                    Some(third) => {
                        match third {
                            Third::Minor => {
                                if let Some(Triad::Diminished) = self.triad {
                                    self.add.push(Interval::Dim5);
                                }
                                self.triad = Some(Triad::Minor);
                            }
                            &Third::Major => self.triad = Some(Triad::Major),
                        }
                    }
                }
            }
            Interval::Aug5 | Interval::Min6 => {
                match &self.third {
                    None => self.add.push(interval),
                    Some(third) => {
                        match third {
                            Third::Minor => return Err("Invalid inversion"),
                            Third::Major => {
                                if let Some(_) = self.triad {
                                    self.add.push(interval);
                                } else {
                                    self.fifth = true;
                                    self.triad = Some(Triad::Augmented);
                                }
                            }
                        }
                    }
                }
            }
            _ => return Err("Invalid interval"),
        }

        Ok(())
    }
}

#[cfg(test)]
mod handle_third_tests {

    use crate::models::{chord::Chord, interval::Interval, third::Third};

    #[test]
    fn min2() {
        let mut a = Chord::new();
        a.handle_third(Interval::Min2);
        
        let mut b = Chord::new();
        b.sus.push(Interval::Min2);

        assert_eq!(a, b);
    }


    #[test]
    fn maj2() {
        let mut a = Chord::new();
        a.handle_third(Interval::Maj2);
        
        let mut b = Chord::new();
        b.sus.push(Interval::Maj2);

        assert_eq!(a, b);
    }


    #[test]
    fn min3() {
        let mut a = Chord::new();
        a.handle_third(Interval::Min3);
        
        let mut b = Chord::new();
        b.third = Some(Third::Minor);

        assert_eq!(a, b);
    }


    #[test]
    fn maj3() {
        let mut a = Chord::new();
        a.handle_third(Interval::Maj3);
        
        let mut b = Chord::new();
        b.third = Some(Third::Major);

        assert_eq!(a, b);
    }


    #[test]
    fn perf4() {
        let mut a = Chord::new();
        a.handle_third(Interval::Perf4);
        
        let mut b = Chord::new();
        b.sus.push(Interval::Perf4);

        assert_eq!(a, b);
    }


    #[test]
    fn aug4() {
        let mut a = Chord::new();
        a.handle_third(Interval::Aug4);
        
        let mut b = Chord::new();
        b.sus.push(Interval::Aug4);

        assert_eq!(a, b);
    }

    #[test]
    fn multiple_sus() {
        let mut a = Chord::new();
        a.handle_third(Interval::Min2);
        a.handle_third(Interval::Maj2);
        a.handle_third(Interval::Perf4);
        a.handle_third(Interval::Aug4);
        
        let mut b = Chord::new();
        b.sus.push(Interval::Min2);
        b.sus.push(Interval::Maj2);
        b.sus.push(Interval::Perf4);
        b.sus.push(Interval::Aug4);

        assert_eq!(a, b);
    }


    #[test]
    fn sus_min3() {
        let mut a = Chord::new();
        a.handle_third(Interval::Maj2);
        a.handle_third(Interval::Min3);
        
        let mut b = Chord::new();
        b.third = Some(Third::Minor);
        b.add.push(Interval::Maj2);

        assert_eq!(a, b);
    }


    #[test]
    fn sus_maj3() {
        let mut a = Chord::new();
        a.handle_third(Interval::Maj2);
        a.handle_third(Interval::Maj3);
        
        let mut b = Chord::new();
        b.third = Some(Third::Major);
        b.add.push(Interval::Maj2);

        assert_eq!(a, b);
    }


    #[test]
    fn sus_min3_add() {
        let mut a = Chord::new();
        a.handle_third(Interval::Maj2);
        a.handle_third(Interval::Min3);
        a.handle_third(Interval::Perf4);
        
        let mut b = Chord::new();
        b.third = Some(Third::Minor);
        b.add.push(Interval::Maj2);
        b.add.push(Interval::Perf4);

        assert_eq!(a, b);
    }


    #[test]
    fn sus_maj3_add() {
        let mut a = Chord::new();
        a.handle_third(Interval::Maj2);
        a.handle_third(Interval::Maj3);
        a.handle_third(Interval::Perf4);
        
        let mut b = Chord::new();
        b.third = Some(Third::Major);
        b.add.push(Interval::Maj2);
        b.add.push(Interval::Perf4);

        assert_eq!(a, b);
    }


    #[test]
    fn sus_min3_maj3() {
        let mut a = Chord::new();
        a.handle_third(Interval::Maj2);
        a.handle_third(Interval::Min3);
        a.handle_third(Interval::Maj3);
        
        let mut b = Chord::new();
        b.third = Some(Third::Major);
        b.add.push(Interval::Maj2);
        b.add.push(Interval::Min3);

        assert_eq!(a, b);
    }


    #[test]
    fn sus_min3_maj3_add() {
        let mut a = Chord::new();
        a.handle_third(Interval::Maj2);
        a.handle_third(Interval::Min3);
        a.handle_third(Interval::Maj3);
        a.handle_third(Interval::Perf4);
        
        let mut b = Chord::new();
        b.third = Some(Third::Major);
        b.add.push(Interval::Maj2);
        b.add.push(Interval::Min3);
        b.add.push(Interval::Perf4);

        assert_eq!(a, b);
    }
}

mod handle_fifth_tests {

}