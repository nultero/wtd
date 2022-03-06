use std::fs::DirEntry;

use crate::dir_search::*;
use crate::search::*;
// use crate::gitignore;

const GITIGNORE: &'static str = ".gitignore";

pub fn find_todos() {
    let cwd = get_dir().expect("could not read current dir");
    let mut v: Vec<DirEntry> = vec![];
    for file in cwd {
        let file = file.unwrap();
        let path = file.path();
        let path_str = path.to_str().unwrap();
        if path_str.ends_with(GITIGNORE) {
            println!(">>>> found gitignore")
        } else {
            v.push(file);
        }
    }

    for f in v {
        search(f);
    }
}
