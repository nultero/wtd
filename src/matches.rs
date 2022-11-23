use std::collections::HashMap;

use crate::fmt::*;
use crate::search;

pub type Matches = HashMap<String, Vec<LineMatch>>;

pub struct LineMatch {
    pub line_no: i32,
    pub priority: i32,
    pub contents: String,
}

pub fn print_matches(map: Matches, short_output: bool, reverse: bool) {

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

    for pv in pvals {
        let mut fname = String::new();
        let opt = prio_map.get(pv);
        match opt {
            Some(f) => { fname = f.to_string(); }
            None => {
                continue;
            }
        }

        if short_output {
            println!(
                "{}s in {}: {}",
                fmt_orange(&search::TODO),
                fmt_underline(&fname),
                pv
            );
            continue;
        }
        
        println!("{}", fmt_underline(&fname));

        let lm: &Vec<LineMatch>;
        let opt = map.get(&fname);
        match opt {
            Some(lmatch) => { lm = lmatch; }
            None => { continue; }
        }

        for m in lm {
            let mut pri: String = m.priority.to_string();   
            match m.priority {
                2..=3  =>  { pri = fmt_blue(   &m.priority.to_string() ) },
                4..=5  =>  { pri = fmt_yellow( &m.priority.to_string() ) },
                6..=7  =>  { pri = fmt_orange( &m.priority.to_string() ) },
                8..    =>  { pri = fmt_red(    &m.priority.to_string() ) },
                _      =>  { },
            }
            let cont: String = m.contents.replace(
                search::TODO,
                &fmt_orange(&search::TODO),
            );
            
            // out = format!("line: {}, priority: {}", m.line_no, pri);
            println!("  line: {}, priority: {}, {}", m.line_no, pri, cont);
            }        
    }
}