use std::fs::DirEntry;
use crate::flags::Flags;
use crate::matches;
use crate::search::*;
use std::{fs::ReadDir, io::Error};
// use crate::gitignore;

const GITIGNORE: &'static str = ".gitignore";

fn get_dir() -> Result<ReadDir, Error> {
    let cwd = std::env::current_dir()?;
    return cwd.read_dir();
}

pub fn find_todos(flags: Flags) {
    let cwd = get_dir().expect("could not read current dir");
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

    for f in dir {
        search(f, &flags, &mut map);
    }

    matches::print_matches(map);
}

fn is_gitignore(file: &DirEntry) -> bool {
    let path = file.path();
    let path_str = path.to_str().unwrap();
    return path_str.ends_with(GITIGNORE);
}