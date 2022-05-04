use std::collections::HashMap;

use crate::fmt::*;

pub type Matches = HashMap<String, Vec<LineMatch>>;

pub struct LineMatch {
    pub line_no: i32,
    pub priority: i32,
    pub contents: String,
}

pub fn print_matches(map: Matches) {

    // let vslice = vec!();
    for (k, v) in &map {
        println!("{}:", fmt_underline(k));
        for lm in v.iter() {

            let mut pri = lm.priority.to_string();

            match lm.priority {
                2..=3  =>  { pri = fmt_blue( &lm.priority.to_string() ) },
                4..=5  =>  { pri = fmt_yellow( &lm.priority.to_string() ) },
                6..=7  =>  { pri = fmt_orange( &lm.priority.to_string() ) },
                8..    =>  { pri = fmt_red( &lm.priority.to_string() ) },
                _      =>  { },
            }

            println!("   line: {}, priority: {}", lm.line_no, pri);
        }
        print!("\n");
    }
}