use std::{fs::ReadDir, io::Error};

pub fn get_dir() -> Result<ReadDir, Error> {
    let cwd = std::env::current_dir()?;
    return cwd.read_dir();
}
