#[allow(dead_code)]
    #[derive(Debug, PartialEq)]
    pub enum Interval {
        Unison,
        Min2,
        Maj2,
        Min3,
        Maj3,
        Perf4,
        Aug4,
        Dim5,
        Perf5,
        Aug5,
        Min6,
        Maj6,
        Dim7,
        Min7,
        Maj7,
        Octave,
        Min9,
        Maj9,
        Min10,
        Maj10,
        Perf11,
        Aug11,
        Dim12,
        Perf12,
        Min13,
        Maj13,
        Min14,
        Maj14,
        DoubleOct,
    }

    impl Interval {
        pub fn as_written(&self) -> Result<&str, String> {
            use Interval::*;
            let text = match self {
                Min2 => "b2",
                Maj2 => "2",
                Min3 => "#2", // add
                // Maj3 => "3",
                Perf4 => "4",
                Aug4 => "#4",
                Dim5 => "b5",
                // Perf5 => "5",
                Aug5 | Min6 => "b6",
                Maj6 => "6",
                Dim7 => "bb7",
                Min7 => "b7",
                Maj7 => "7",
                Min9 => "b9",
                Maj9 => "9",
                // Min10 => "b10",
                // Maj10 => "10",
                Perf11 => "11",
                Aug11 | Dim12 => "#11",
                Min13 => "b13",
                Maj13 => "13",
                // Min14 => "b14",
                // Maj14 => "14",
                _ => return Err(format!("Cannot write {:#?}", self))
            };

            Ok(text)
        }
    }