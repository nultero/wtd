use std::fs::DirEntry;
use std::path::Path;
use std::path::PathBuf;
use crate::flags::Flags;
use crate::matches;
use crate::search::*;
use std::{fs::ReadDir};
// use crate::gitignore;

const GITIGNORE: &'static str = ".gitignore";

fn get_dir() -> (PathBuf, ReadDir) {
    let pbuf = std::env::current_dir().unwrap();
    let cwd = pbuf.read_dir().expect("could not read current dir");
    return (pbuf, cwd);
}

fn abspath_from_direntry(de: &DirEntry) -> String {
    let p = de.path().canonicalize().unwrap();
    return p.to_str().unwrap().to_owned();
}

/// Main entrypoint to the file search function.
pub fn find_todos(flags: Flags) {

    // TODOOOO clean up all this func's code

    let (pbuf, cwd) = get_dir();
    let mut dir: Vec<DirEntry> = vec!();

    let mut is_empty = true;
    let mut igf_pathstr = "".to_string();

    for file in cwd {
        match file {
            Ok(file) => {
                if is_gitignore(&file) {
                    igf_pathstr = abspath_from_direntry(&file);
                    is_empty = false;
                    
                } else {
                    dir.push(file);
                }
            },
            Err(_e) => {}
        }
    }
    
    let igf_path = Path::new(&igf_pathstr);
    let igf = gitignore::File::new(&igf_path).unwrap();

    let mut map = matches::Matches::new();

    let path_name = pbuf.to_str().unwrap().to_owned();

    for f in dir {
        if !is_empty {

            let excluded = igf.is_excluded(f.path().as_path());
            match excluded {
                Ok(excluded) => {
                    if !excluded {
                        search(path_name.to_string(), f, &flags, &mut map);
                    }
                },
                Err(_) => {}, 
            }

        } else if is_empty {
            search(path_name.to_string(), f, &flags, &mut map);
        }
    }
    
    matches::print_matches(map);
}

fn is_gitignore(file: &DirEntry) -> bool {
    let path = file.path();
    let path_str = path.to_str().unwrap();
    return path_str.ends_with(GITIGNORE);
}