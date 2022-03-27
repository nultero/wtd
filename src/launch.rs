use std::fs::DirEntry;
use crate::dir_search::*;
use crate::flags::Flags;
use crate::matches;
use crate::search::*;
// use crate::gitignore;

const GITIGNORE: &'static str = ".gitignore";

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

    let mut m = matches::Matches::new();

    for f in dir {
        search(f, &flags, &mut m);
    }
}

fn is_gitignore(file: &DirEntry) -> bool {
    let path = file.path();
    let path_str = path.to_str().unwrap();
    return path_str.ends_with(GITIGNORE);
}