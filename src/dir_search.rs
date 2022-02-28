// use dir;

pub fn search() {
    let cwd = std::env::current_dir();
    let cwd = cwd.unwrap();

    let dir = cwd.read_dir();
    let dir = dir.unwrap();

    for fl in dir {
        println!("{:?}", fl);
    }

    let cwd = cwd.to_str().unwrap();
    println!("{}", cwd);
}