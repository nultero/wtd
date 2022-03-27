pub struct Flags {
    help: bool,
    nostrip: bool,
    reverse: bool,
    verbose: i8,
}

pub fn standard_flags() -> Flags {
    return Flags{ 
        help: false, 
        nostrip: false, 
        reverse: false, 
        verbose: 0,
    }
}