use std::{
    fs::{read_to_string, DirEntry},
    path::PathBuf,
};

use crate::{
    flags::Flags,
    matches::{LineMatch, Matches},
};

const TODO: &'static str = "TODO";
const OCHAR: u8 = 79;

pub fn search(f: DirEntry, flags: &Flags, matches: &mut Matches) {
    let (f, fname, ferr) = get_pathbuf(&f);

    if !f.is_dir() {
        let conts = read_to_string(f).expect(&ferr);
        for (i, line) in conts.lines().enumerate() {
            if line.contains(&TODO) {
                let (count, _idx) = get_priority(line);
                let tmp_str = String::from("");

                if flags.verbose > 0 {
                    println!("haven't got this far");
                }

                let line_num = (i + 1) as i32;
                let lm = LineMatch{ line_no: line_num, priority: count, contents: tmp_str };

                if matches.contains_key(&fname) {
                    let v = matches.get_mut(&fname).unwrap();
                    v.push(lm);
                } else {
                    let v = vec!(lm);
                    matches.insert(fname.to_string(), v);
                }
            }
        }
    } else {
        // recurse over subdir
    }
}

/// Sugar over unwrapping across a couple types. Also returns err msg.
fn get_pathbuf(f: &DirEntry) -> (PathBuf, String, String) {
    let n = f.file_name();
    let fname = n.to_str().unwrap();
    let ferr = format!("could not read/open {}", fname);
    let pathbuf = f.path().canonicalize().expect(&ferr);
    return (pathbuf, fname.to_owned(), ferr);
}

/// Counts the capital-O chars on the end of the first TODO on the matching found line.
/// Also returns the index of the start of the TODO for capture for standard flag
/// that strips left space/garbage.
fn get_priority(line: &str) -> (i32, i32) {
    let mut count: i32 = 0;
    let bytes = line.as_bytes();
    let comp = TODO.as_bytes();
    let mut idx: i32 = -1;

    for (i, b) in bytes.iter().enumerate() {
        if b == &OCHAR && i > 2 {
            let slice = bytes.get(i - 3..i + 1).unwrap();
            if slice == comp {
                idx = (i - 3) as i32;
                let mut i = i;
                while i < bytes.len() && bytes[i] == OCHAR {
                    count += 1;
                    i     += 1;
                }
                break;
            }
        }
    }

    return (count, idx);
}
