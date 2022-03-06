use std::fs::{read_to_string, DirEntry};

const TODO: &'static str = "TODO";

pub fn search(f: DirEntry) {
    let fname = f.file_name();
    let fname = fname.to_str().unwrap();
    let ferr = format!("could not open {}", fname);

    let f = f.path().canonicalize().expect(&ferr);

    if !f.is_dir() {
        let content = read_to_string(f).expect(&ferr);

        for ln in content.lines() {
            if ln.contains(&TODO) {
                println!("{}", &ln);
            }
        }
    }
}
