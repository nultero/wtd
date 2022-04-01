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

pub fn search(parent_dir: String, f: DirEntry, flags: &Flags, matches: &mut Matches) {
    let (f, fname, _ferr) = get_pathbuf(&f);

    if !f.is_dir() {

        let mut conts = String::new();

        let contents = read_to_string(f);
        match contents {
            Ok(c) => { conts = c },
            Err(e) => { println!("{}: {}", &fname,  e) },
        }


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

        
    } else if f.is_dir() {
        if fname == ".git" {
            return;
        }

        let subpath = format!("{}/{}", parent_dir, fname);

        let subdir = f.read_dir();

        match subdir {
            Ok(sub) => {
                for d in sub {
                    let d = d.unwrap();
                    search(subpath.to_string(), d, flags, matches);
                }
            },

            Err(_e) => {}
        }

        // TODO recurse over subdir
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