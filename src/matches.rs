use std::collections::HashMap;

pub type Matches = HashMap<String, Vec<LineMatch>>;

pub struct LineMatch {
    pub line_no: i32,
    pub priority: i32,
    pub contents: String,
}

pub fn print_matches(map: Matches) {
    // let vslice = vec!();
    for (k, v) in &map {
        println!("{}:", k);
        for lm in v.iter() {
            println!("   line: {}, priority: {}", lm.line_no, lm.priority);
        }
        print!("\n");
    }
}