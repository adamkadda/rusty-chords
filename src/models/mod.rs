mod chord;
mod interval;

mod third {
    #[allow(dead_code)]
    #[derive(Debug, PartialEq)]
    pub enum Third {
        Minor,
        Major,
    }
}

mod triad {
    #[allow(dead_code)]
    #[derive(Debug, PartialEq)]
    pub enum Triad {
        Minor,
        Major,
        Diminished,
        Augmented,
    }
}

mod fifth {
    #[allow(dead_code)]
    #[derive(Debug, PartialEq)]
    pub enum Fifth {
        Diminished,
        Perfect,
        Augmented,
    }
}