use std::{fs::ReadDir, io::Error};

// use crate::ignore::*;

// pub fn search(dir: ReadDir) {
//     for file in dir {
//         let file = file.unwrap();

//         check_against_igs(file);

//         if file.path().is_dir() {
//             let nxt_dir = file.path().read_dir().unwrap();
//             search(nxt_dir);
//         } else {
//             println!("plunk: {}", search_file(file));
//         }
//     }
// }

pub fn get_dir() -> Result<ReadDir, Error> {
    let cwd = std::env::current_dir()?;
    return cwd.read_dir();
}

// fn search_file(file: DirEntry) -> String {
//     return file.path().to_str().unwrap().to_owned();
// }
