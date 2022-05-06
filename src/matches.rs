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

    let mut prio_map: HashMap<i32, &String> = HashMap::new();
    for (k, lm) in &map {
        let mut sum = 0;
        for m in lm {
            sum += m.priority;
        }
        prio_map.insert(sum, k);
    }

    let mut pvals: Vec<&i32> = prio_map.keys().collect();
    pvals.sort();

    if reverse {
        pvals.reverse();
    }


    // -v     prints the line number of each TODO
    // -vv    prints the individual priorities of each TODO
    // -vvv   prints the entire line of each TODO

    for pv in pvals {
        let fname = prio_map.get(pv).unwrap();
        if verbosity == 0 {
            println!(
                "{}s in {}: {}",
                fmt_orange(search::TODO),
                fmt_underline(fname),
                pv
            );

        } else {
            println!("{}:", fmt_underline(fname));
            let lm = map.get(fname.clone()).unwrap();
            for m in lm {

                let mut out = String::new();

                if verbosity >= 1 {
                    out = format!("line: {}", m.line_no);
                }
                
                if verbosity >= 2 {
                    let mut pri = m.priority.to_string();   
                    match m.priority {
                        2..=3  =>  { pri = fmt_blue(   &m.priority.to_string() ) },
                        4..=5  =>  { pri = fmt_yellow( &m.priority.to_string() ) },
                        6..=7  =>  { pri = fmt_orange( &m.priority.to_string() ) },
                        8..    =>  { pri = fmt_red(    &m.priority.to_string() ) },
                        _      =>  { },
                    }

                    out = format!("{}, priority: {}", out, pri);
                }
                
                if verbosity == 3 {
                    let c = m.contents.replace(
                        search::TODO,
                        &fmt_orange(search::TODO),
                    );
                    out = format!("{} | {}", out, c);
                }

                println!("  {}", out);
            }
        }
    }
}