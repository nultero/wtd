use std::fs::{read_to_string, DirEntry};

use crate::{flags::Flags, launch::MatchedLines};

const TODO: &'static str = "TODO";

pub fn search(f: DirEntry, flags: &Flags, matches: &mut MatchedLines) {
    
    let fname = f.file_name();
    let fname = fname.to_str().unwrap();
    let ferr = format!("could not open {}", fname);

    let f = f.path().canonicalize().expect(&ferr);

    if !f.is_dir() {
        let content = read_to_string(f).expect(&ferr);

        for (i, ln) in content.lines().enumerate() {
            if ln.contains(&TODO) {
                println!("{}", &ln);
            }
        }
    }
}
