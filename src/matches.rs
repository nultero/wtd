#[allow(dead_code)]
pub struct Matches {
    files: Vec<File>,
}

#[allow(dead_code)]
pub struct File {
    name: String,
    lines: Vec<Line>,
}

pub struct Line {
    pub line_no: i32,
    pub priority: i32,
}

impl Matches {
    pub fn add(&self, file_name: String, line_no: i32, priority: i32) {


        
    }

    pub fn new() -> Matches {
        return Matches { files: vec!() }
    }
}