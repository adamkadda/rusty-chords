use super::{fifth::Fifth, interval::Interval, seventh::Seventh, third::Third, triad::Triad};

#[derive(Debug, PartialEq)]
pub struct Chord {
    third: Option<Third>,
    fifth: Option<Fifth>,
    seventh: Option<Seventh>,
    ninth: bool,
    eleventh: bool,
    triad: Option<Triad>,
    sus: Vec<Interval>,
    add: Vec<Interval>,
    lead: Option<Interval>,
}

#[allow(dead_code)]
impl Chord {
    pub fn new() -> Chord {
        Chord {
            third: None,
            fifth: None,
            seventh: None,
            ninth: false,
            eleventh: false,
            triad: None,
            sus: Vec::new(),
            add: Vec::new(),
            lead: None,
        }
    }


    fn handle_third(&mut self, interval: Interval) -> Result<(), &str> {
        match interval {
            Interval::Min3 => {
                self.third = Some(Third::Minor);
                if !self.sus.is_empty() { self.add.append(&mut self.sus); }
            }
            Interval::Maj3 => {
                if let Some(Third::Minor) = self.third { self.add.push(Interval::Min3); }
                self.third = Some(Third::Major);
                if !self.sus.is_empty() { self.add.append(&mut self.sus); }
            }
            Interval::Min2 | Interval::Maj2 | Interval::Perf4 => {
                if let None = self.third {
                    self.sus.push(interval)
                } else {
                    self.add.push(interval)
                }
            }
            _ => return Err("Invalid interval"),
        }

        Ok(())
    }


    fn handle_fifth(&mut self, interval: Interval) -> Result<(), &str>{
        match interval {
            Interval::Aug4 | Interval::Dim5 => {
                match &self.third {
                    None => self.sus.push(interval),
                    Some(third) => {
                        match third {
                            Third::Minor => {
                                self.fifth = Some(Fifth::Diminished);
                                self.triad = Some(Triad::Diminished)
                            }
                            Third::Major => self.add.push(interval),
                        }
                    }
                }
            }
            Interval::Perf5 => {
                self.fifth = Some(Fifth::Perfect);
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
                match self.fifth {
                    None => {
                        if let Some(third) = &self.third {
                            match third {
                                Third::Major => self.triad = Some(Triad::Augmented),
                                Third::Minor => return Err("Invalid inversion"),
                            }
                        }
                        self.fifth = Some(Fifth::Augmented);
                    }
                    Some(_) => self.add.push(Interval::Min6), // dim triad | min triad
                }
            }
            _ => return Err("Invalid interval"),
        }

        Ok(())
    }


    fn handle_sixth(&mut self, interval: Interval) -> Result<(), &str> {
        match interval {
            Interval::Maj6 => {
                if let Some(Triad::Diminished) = &self.triad {
                    self.seventh = Some(Seventh::Diminished);
                    self.lead = Some(Interval::Dim7);
                } else { self.add.push(Interval::Maj6); }
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
    fn invalid_interval() {
        let mut a = Chord::new();
        assert_eq!(a.handle_third(Interval::Aug4), Err("Invalid interval"));
    }

    #[test]
    fn min2() {
        let mut a = Chord::new();
        let _ = a.handle_third(Interval::Min2);
        
        let mut b = Chord::new();
        b.sus.push(Interval::Min2);

        assert_eq!(a, b);
    }


    #[test]
    fn maj2() {
        let mut a = Chord::new();
        let _ = a.handle_third(Interval::Maj2);
        
        let mut b = Chord::new();
        b.sus.push(Interval::Maj2);

        assert_eq!(a, b);
    }


    #[test]
    fn min3() {
        let mut a = Chord::new();
        let _ = a.handle_third(Interval::Min3);
        
        let mut b = Chord::new();
        b.third = Some(Third::Minor);

        assert_eq!(a, b);
    }


    #[test]
    fn maj3() {
        let mut a = Chord::new();
        let _ = a.handle_third(Interval::Maj3);
        
        let mut b = Chord::new();
        b.third = Some(Third::Major);

        assert_eq!(a, b);
    }


    #[test]
    fn perf4() {
        let mut a = Chord::new();
        let _ = a.handle_third(Interval::Perf4);
        
        let mut b = Chord::new();
        b.sus.push(Interval::Perf4);

        assert_eq!(a, b);
    }


    #[test]
    fn multiple_sus() {
        let mut a = Chord::new();
        let _ = a.handle_third(Interval::Min2);
        let _ = a.handle_third(Interval::Maj2);
        let _ = a.handle_third(Interval::Perf4);
        
        let mut b = Chord::new();
        b.sus.push(Interval::Min2);
        b.sus.push(Interval::Maj2);
        b.sus.push(Interval::Perf4);

        assert_eq!(a, b);
    }


    #[test]
    fn sus_min3() {
        let mut a = Chord::new();
        let _ = a.handle_third(Interval::Maj2);
        let _ = a.handle_third(Interval::Min3);
        
        let mut b = Chord::new();
        b.third = Some(Third::Minor);
        b.add.push(Interval::Maj2);

        assert_eq!(a, b);
    }


    #[test]
    fn sus_maj3() {
        let mut a = Chord::new();
        let _ = a.handle_third(Interval::Maj2);
        let _ = a.handle_third(Interval::Maj3);
        
        let mut b = Chord::new();
        b.third = Some(Third::Major);
        b.add.push(Interval::Maj2);

        assert_eq!(a, b);
    }


    #[test]
    fn sus_min3_add() {
        let mut a = Chord::new();
        let _ = a.handle_third(Interval::Maj2);
        let _ = a.handle_third(Interval::Min3);
        let _ = a.handle_third(Interval::Perf4);
        
        let mut b = Chord::new();
        b.third = Some(Third::Minor);
        b.add.push(Interval::Maj2);
        b.add.push(Interval::Perf4);

        assert_eq!(a, b);
    }


    #[test]
    fn sus_maj3_add() {
        let mut a = Chord::new();
        let _ = a.handle_third(Interval::Maj2);
        let _ = a.handle_third(Interval::Maj3);
        let _ = a.handle_third(Interval::Perf4);
        
        let mut b = Chord::new();
        b.third = Some(Third::Major);
        b.add.push(Interval::Maj2);
        b.add.push(Interval::Perf4);

        assert_eq!(a, b);
    }


    #[test]
    fn sus_min3_maj3() {
        let mut a = Chord::new();
        let _ = a.handle_third(Interval::Maj2);
        let _ = a.handle_third(Interval::Min3);
        let _ = a.handle_third(Interval::Maj3);
        
        let mut b = Chord::new();
        b.third = Some(Third::Major);
        b.add.push(Interval::Maj2);
        b.add.push(Interval::Min3);

        assert_eq!(a, b);
    }


    #[test]
    fn sus_min3_maj3_add() {
        let mut a = Chord::new();
        let _ = a.handle_third(Interval::Maj2);
        let _ = a.handle_third(Interval::Min3);
        let _ = a.handle_third(Interval::Maj3);
        let _ = a.handle_third(Interval::Perf4);
        
        let mut b = Chord::new();
        b.third = Some(Third::Major);
        b.add.push(Interval::Maj2);
        b.add.push(Interval::Min3);
        b.add.push(Interval::Perf4);

        assert_eq!(a, b);
    }
}

#[cfg(test)]
mod handle_fifth_tests {
    use crate::models::{chord::Chord, fifth::Fifth, interval::Interval, third::Third, triad::Triad};

    #[test]
    fn invalid_interval() {
        let mut a = Chord::new();
        assert_eq!(a.handle_fifth(Interval::Unison), Err("Invalid interval"));
    }


    #[test]
    fn aug4_alone() { // sus
        let mut a = Chord::new();
        let _ = a.handle_fifth(Interval::Aug4);

        let mut b = Chord::new();
        b.sus.push(Interval::Aug4);

        assert_eq!(a, b);
    }


    #[test]
    fn min3_aug4() { // diminished triad
        let mut a = Chord::new();
        let _ = a.handle_third(Interval::Min3);
        let _ = a.handle_fifth(Interval::Aug4);

        let mut b = Chord::new();
        b.third = Some(Third::Minor);
        b.fifth = Some(Fifth::Diminished);
        b.triad = Some(Triad::Diminished);

        assert_eq!(a, b);
    }


    #[test]
    fn maj3_aug4() { // weird one, add, no triad
        let mut a = Chord::new();
        let _ = a.handle_third(Interval::Maj3);
        let _ = a.handle_fifth(Interval::Aug4);

        let mut b = Chord::new();
        b.third = Some(Third::Major);
        b.fifth = Some(Fifth::Diminished);
    }


    #[test]
    fn perf5_alone() {
        let mut a = Chord::new();
        let _ = a.handle_fifth(Interval::Perf5);

        let mut b = Chord::new();
        b.fifth = Some(Fifth::Perfect);

        assert_eq!(a, b);
    }


    #[test]
    fn min3_perf5() {
        let mut a = Chord::new();
        let _ = a.handle_third(Interval::Min3);
        let _ = a.handle_fifth(Interval::Perf5);

        let mut b = Chord::new();
        b.third = Some(Third::Minor);
        b.fifth = Some(Fifth::Perfect);
        b.triad = Some(Triad::Minor);

        assert_eq!(a, b);
    }


    #[test]
    fn min3_dim5_perf5() { // dim triad -> minor triad
        let mut a = Chord::new();
        let _ = a.handle_third(Interval::Min3);
        let _ = a.handle_fifth(Interval::Dim5);
        let _ = a.handle_fifth(Interval::Perf5);

        let mut b = Chord::new();
        b.third = Some(Third::Minor);
        b.fifth = Some(Fifth::Perfect);
        b.triad = Some(Triad::Minor);
        b.add.push(Interval::Dim5);

        assert_eq!(a, b);
    }


    #[test]
    fn maj3_perf5() {
        let mut a = Chord::new();
        let _ = a.handle_third(Interval::Maj3);
        let _ = a.handle_fifth(Interval::Perf5);

        let mut b = Chord::new();
        b.third = Some(Third::Major);
        b.fifth = Some(Fifth::Perfect);
        b.triad = Some(Triad::Major);

        assert_eq!(a, b);
    }


    #[test]
    fn aug5_alone() {
        let mut a = Chord::new();
        let _ = a.handle_fifth(Interval::Aug5);

        let mut b = Chord::new();
        b.fifth = Some(Fifth::Augmented);

        assert_eq!(a, b);
    }


    #[test]
    fn perf5_aug5() {
        let mut a = Chord::new();
        let _ = a.handle_fifth(Interval::Perf5);
        let _ = a.handle_fifth(Interval::Aug5);

        let mut b = Chord::new();
        b.fifth = Some(Fifth::Perfect);
        b.add.push(Interval::Min6);

        assert_eq!(a, b);
    }


    #[test]
    fn min3_perf5_aug5() {
        let mut a = Chord::new();
        let _ = a.handle_third(Interval::Min3);
        let _ = a.handle_fifth(Interval::Perf5);
        let _ = a.handle_fifth(Interval::Aug5);

        let mut b = Chord::new();
        b.third = Some(Third::Minor);
        b.fifth = Some(Fifth::Perfect);
        b.triad = Some(Triad::Minor);
        b.add.push(Interval::Min6);

        assert_eq!(a, b);
    }


    #[test]
    fn min3_aug5() {
        let mut a = Chord::new();
        let _ = a.handle_third(Interval::Min3);
        let res = a.handle_fifth(Interval::Aug5);

        assert_eq!(res, Err("Invalid inversion"));
    }


    #[test]
    fn maj3_perf5_aug5() {
        let mut a = Chord::new();
        let _ = a.handle_third(Interval::Maj3);
        let _ = a.handle_fifth(Interval::Perf5);
        let _ = a.handle_fifth(Interval::Aug5);

        let mut b = Chord::new();
        b.third = Some(Third::Major);
        b.fifth = Some(Fifth::Perfect);
        b.triad = Some(Triad::Major);
        b.add.push(Interval::Min6);

        assert_eq!(a, b);
    }


    #[test]
    fn maj3_aug5() {
        let mut a = Chord::new();
        let _ = a.handle_third(Interval::Maj3);
        let _ = a.handle_fifth(Interval::Aug5);

        let mut b = Chord::new();
        b.third = Some(Third::Major);
        b.fifth = Some(Fifth::Augmented);
        b.triad = Some(Triad::Augmented);

        assert_eq!(a, b);
    }
}

#[cfg(test)]
mod handle_sixth_tests {
    use crate::models::{chord::Chord, fifth::Fifth, interval::Interval, seventh::Seventh, third::Third, triad::Triad};

    #[test]
    fn invalid_interval() {
        let mut a = Chord::new();
        assert_eq!(a.handle_sixth(Interval::Unison), Err("Invalid interval"));
    }


    #[test]
    fn dim_triad() {
        let mut a = Chord::new();
        let _ = a.handle_third(Interval::Min3);
        let _ = a.handle_fifth(Interval::Dim5);
        let _ = a.handle_sixth(Interval::Maj6);

        let mut b = Chord::new();
        b.third = Some(Third::Minor);
        b.fifth = Some(Fifth::Diminished);
        b.triad = Some(Triad::Diminished);
        b.seventh = Some(Seventh::Diminished);
        b.lead = Some(Interval::Dim7);

        assert_eq!(a, b);
    }


    #[test]
    fn min_triad() {
        let mut a = Chord::new();
        let _ = a.handle_third(Interval::Min3);
        let _ = a.handle_fifth(Interval::Perf5);
        let _ = a.handle_sixth(Interval::Maj6);

        let mut b = Chord::new();
        b.third = Some(Third::Minor);
        b.fifth = Some(Fifth::Perfect);
        b.triad = Some(Triad::Minor);
        b.add.push(Interval::Maj6);

        assert_eq!(a, b);
    }


    #[test]
    fn maj_triad() {
        let mut a = Chord::new();
        let _ = a.handle_third(Interval::Maj3);
        let _ = a.handle_fifth(Interval::Perf5);
        let _ = a.handle_sixth(Interval::Maj6);

        let mut b = Chord::new();
        b.third = Some(Third::Major);
        b.fifth = Some(Fifth::Perfect);
        b.triad = Some(Triad::Major);
        b.add.push(Interval::Maj6);

        assert_eq!(a, b);
    }


    #[test]
    fn aug_triad() {
        let mut a = Chord::new();
        let _ = a.handle_third(Interval::Maj3);
        let _ = a.handle_fifth(Interval::Aug5);
        let _ = a.handle_sixth(Interval::Maj6);

        let mut b = Chord::new();
        b.third = Some(Third::Major);
        b.fifth = Some(Fifth::Augmented);
        b.triad = Some(Triad::Augmented);
        b.add.push(Interval::Maj6);

        assert_eq!(a, b);
    }


    #[test]
    fn no_triad() {
        let mut a = Chord::new();
        let _ = a.handle_third(Interval::Maj2);
        let _ = a.handle_fifth(Interval::Perf5);
        let _ = a.handle_sixth(Interval::Maj6);

        let mut b = Chord::new();
        b.sus.push(Interval::Maj2);
        b.fifth = Some(Fifth::Perfect);
        b.add.push(Interval::Maj6);

        assert_eq!(a, b);
    }
}