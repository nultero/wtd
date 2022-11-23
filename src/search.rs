use std::{
    fs::{read_to_string, DirEntry},
    path::PathBuf,
};


use crate::{
    flags::Flags,
    fmt,
    matches::{LineMatch, Matches}, launch::Excludes,
};

pub const TODO: &'static str = "TODO";
const OCHAR: u8 = 79;

pub fn search_dir<T>(
    dir: Vec<DirEntry>,
    flags: &Flags,
    matches: &mut Matches,
    path_name: String,
    ig: T, 
) 
where T: Excludes
{
    for f in dir {
        let excluded = ig.is_excluded(f.path().as_path());
        match excluded {
            Ok(excluded) => {
                if !excluded {
                    search(path_name.to_string(), f, &flags, matches);
                }
            },
            Err(_) => {}, 
        }
    }
}

pub fn search(parent_dir: String, f: DirEntry, flags: &Flags, matches: &mut Matches) {

    let (f, fname, ferr) = get_pathbuf(&f);
    if ferr.len() != 0 {
        println!("{}", ferr);
        return;
    }

    if !f.is_dir() {

        let mut conts = String::new();
        let contents = read_to_string(f);
        match contents {
            Ok(c) => { conts = c },
            Err(e) => { 
                println!("{}: {}", &fname,  e); 
                return; 
            },
        }


        for (i, line) in conts.lines().enumerate() {
            if line.contains(&TODO) {
                let (count, idx) = get_priority(line);
                
                let mut tmp_str = String::new();
                if !flags.nostrip {
                    let opt = line.get(idx..line.len());
                    match opt {
                        Some(ts) => { tmp_str = ts.to_string(); }
                        None => { continue; }
                    }
                    
                } else {
                    tmp_str = line.to_owned();
                }

                let line_num = (i + 1) as i32;
                let lm = LineMatch{ 
                    line_no: line_num, 
                    priority: count, 
                    contents: tmp_str 
                };

                if matches.contains_key(&fname) {
                    let v: &mut Vec<LineMatch>;
                    let opt = matches.get_mut(&fname);
                    match opt {
                        Some(val) => { v = val; }
                        None => { continue; }
                    }
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
                    match d {
                        Ok(d) => {
                            search(subpath.to_string(), d, flags, matches);
                        }

                        Err(e) => {println!(
                                "problem in subdir {}: {}",
                                fmt::fmt_underline(&subpath),
                                &e)}
                    }
                }
            },

            Err(_e) => {}
        }
    }
}


fn get_pathbuf(f: &DirEntry) -> (PathBuf, String, String) {
    let n = f.file_name();
    let mut fname = String::new();
    let mut ferr = String::new();
    let opt = n.to_str();
    match opt {
        Some(s) => { fname = s.to_owned(); }
        None => { ferr = format!("could not read/open {}", fname)}
    }
    let pathbuf = f.path().canonicalize().expect(&ferr);
    return (pathbuf, fname.to_owned(), ferr);
}

/// Counts the capital-O chars on the end of the first TODO on the matching found line.
/// Also returns the index of the start of the TODO for capture for standard flag
/// that strips left space/garbage.
fn get_priority(line: &str) -> (i32, usize) {
    let mut count: i32 = 0;
    let bytes = line.as_bytes();
    let comp = TODO.as_bytes();
    let mut idx: usize = usize::MIN;

    for (i, b) in bytes.iter().enumerate() {
        if b == &OCHAR && i > 2 {
            let slice: &[u8];
            let opt = bytes.get(i - 3..i + 1);
            match opt {
                Some(bytes) => { slice = bytes; }
                None => { break;}
            }
            
            if slice == comp {
                idx = i - 3;
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