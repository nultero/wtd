use std::collections::HashMap;

use crate::fmt::*;
use crate::search;

pub type Matches = HashMap<String, Vec<LineMatch>>;

pub struct LineMatch {
    pub line_no: i32,
    pub priority: i32,
    pub contents: String,
}

pub fn print_matches(map: Matches, verbosity: i8, reverse: bool) {

    if verbosity == 0 {
        let mut prio_map: HashMap<i32, &String> = HashMap::new();
        for (k, lm) in &map {
            let mut sum = 0;
            for m in lm {
                sum += m.priority;
            }
            prio_map.insert(sum, k);
        }

        let mut vals: Vec<&i32> = prio_map.keys().collect();
        vals.sort();
        if reverse {
            vals.reverse();
        }

        for val in vals {
            let fname = prio_map.get(val).unwrap();
            println!(
                "{}s in {}: {}",
                fmt_orange(search::TODO),
                fmt_underline(fname),
                val
            );
        }

    } else if verbosity != 0 {

    }



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