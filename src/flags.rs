pub struct Flags {
    pub help: bool,
    pub nostrip: bool,
    pub reverse: bool,
    pub verbose: i8,
}

pub fn standard_flags() -> Flags {
    return Flags{ 
        help: false, 
        nostrip: false, 
        reverse: false, 
        verbose: 0,
    }
}