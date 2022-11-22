pub struct Flags {
    pub help:    bool,
    pub nostrip: bool,
    pub reverse: bool,
    pub short:   bool,
}

pub fn standard_flags() -> Flags {
    return Flags{ 
        help:    false, 
        nostrip: false, 
        reverse: false, 
        short:   false,
    }
}