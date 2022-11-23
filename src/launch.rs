use std::fs::DirEntry;
use std::path::Path;
use std::path::PathBuf;
use gitignore::File;

use crate::flags::Flags;
use crate::matches;
use crate::search::*;
use std::fs::ReadDir;

const GITIGNORE: &'static str = ".gitignore";
const CWDERR: &'static str = "couldn't read current dir";

fn get_dir() -> (PathBuf, ReadDir) {
    let pbuf = std::env::current_dir().expect(CWDERR);
    let cwd = pbuf.read_dir().expect(CWDERR);
    return (pbuf, cwd);
}

fn abspath_from_direntry(de: &DirEntry) -> String {
    let p = de.path().canonicalize().expect("error canonicalizing path");
    let opt = p.to_str();
    match opt {
        Some(s) => {
            return s.to_owned();
        }
        None => {
            println!("fuck");
            panic!("issue getting absolute paths");
        }
    }
}

struct EmptyIgnore {}
pub trait Excludes {
    fn is_excluded(&self, path: &Path) -> Result<bool, gitignore::Error>;
}
impl Excludes for EmptyIgnore {
    fn is_excluded(&self, _path: &Path) -> Result<bool, gitignore::Error> {
        return Ok(false);
    }
}

impl<'a> Excludes for File<'a> {
    fn is_excluded<'b>(&self, path: &'b Path) -> Result<bool, gitignore::Error> {
        return self.is_excluded(path)
    }
}

/// Main entrypoint to the file search function.
pub fn find_todos(flags: Flags) {

    let (pbuf, cwd) = get_dir();
    let mut dir: Vec<DirEntry> = vec!();

    let mut igf_pathstr = "".to_string();

    for file in cwd {
        match file {
            Ok(file) => {
                if is_gitignore(&file) {
                    igf_pathstr = abspath_from_direntry(&file);
                    
                } else {
                    dir.push(file);
                }
            },
            Err(_) => { continue; } // purposefully throw away
        }
    }
    
    let mut map = matches::Matches::new();

    let mut path_name = String::new();
    let opt = pbuf.to_str();
    match opt {
        Some(s) => { path_name = s.to_owned(); }
        None => {
            println!("what the fuck");
            panic!("not able to convert pathbuf properly");
        }
    }
    
    let igf_path = Path::new(&igf_pathstr);
    let igf = File::new(&igf_path);
    match igf {
        Ok(ig) => {
            search_dir(dir, &flags, &mut map, path_name, ig)
        },
        Err(_) => {
            let ig = EmptyIgnore{};
            search_dir(dir, &flags, &mut map, path_name, ig)
        }
    }    
    // for f in dir {
    //     if !is_empty {
    //         let excluded = ig.is_excluded(f.path().as_path());
    //         match excluded {
    //             Ok(excluded) => {
    //                 if !excluded {
    //                     search(path_name.to_string(), f, &flags, &mut map);
    //                 }
    //             },
    //             Err(_) => {}, 
    //         }
            
    //     } else if is_empty {
    //         search(path_name.to_string(), f, &flags, &mut map);
    //     }
    // }
    matches::print_matches(map, flags.short, flags.reverse);
}

fn is_gitignore(file: &DirEntry) -> bool {
    let path = file.path();
    let mut path_str = String::new();
    let opt = path.to_str();
    match opt {
        Some(s) => {
            path_str = s.to_owned();
        }
        None => {
            return false;
        }
    }
    return path_str.ends_with(GITIGNORE);
}