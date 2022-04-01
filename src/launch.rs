use std::fs::DirEntry;
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

pub fn find_todos(flags: Flags) {
    let (pbuf, cwd) = get_dir();
    let mut dir: Vec<DirEntry> = vec!();

    for file in cwd {
        let file = file.unwrap();
        if is_gitignore(&file) {
            println!(">>>> found gitignore");
        } else {
            dir.push(file);
        }
    }

    let mut map = matches::Matches::new();

    let path_name = pbuf.to_str().unwrap().to_owned();

    for f in dir {
        search(path_name.to_string(), f, &flags, &mut map);
    }

    matches::print_matches(map);
}

fn is_gitignore(file: &DirEntry) -> bool {
    let path = file.path();
    let path_str = path.to_str().unwrap();
    return path_str.ends_with(GITIGNORE);
}