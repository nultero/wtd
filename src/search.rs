use std::fs::{read_to_string, DirEntry};

use crate::{flags::Flags, matches::Matches};

const TODO: &'static str = "TODO";
const OCHAR: u8 = 79;

pub fn search(f: DirEntry, _flags: &Flags, matches: &mut Matches) {
    let fname = f.file_name();
    let fname = fname.to_str().unwrap();
    let ferr = format!("could not read/open {}", fname);
    let f = f.path().canonicalize().expect(&ferr);

    if !f.is_dir() {
        let content = read_to_string(f).expect(&ferr);
        for (i, line) in content.lines().enumerate() {
            if line.contains(&TODO) {
                let count = get_priority(line);


                // matches.
                println!("{}, line: {}, pri_count: {}", &line, i+1, count);
            }
        }
    } else {
        // recurse over subdir
    }
}

/// Counts the capital-O chars on the end of the first TODO on the matching found line.
fn get_priority(line: &str) -> i32 {
    let mut count: i32 = 0;
    let bytes = line.as_bytes();
    let comp = TODO.as_bytes();
    for (i, b) in bytes.iter().enumerate() {
        if b == &OCHAR && i > 2 {
            let slice = bytes.get(i-3..i+1).unwrap();
            if slice == comp {
                let mut i = i;
                while i < bytes.len() && bytes[i] == OCHAR {
                    count += 1;
                    i     += 1;
                }
                break;
            }
        }
    }
    return count;
}