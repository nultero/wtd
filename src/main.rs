use std::env;
mod dir_search;
mod ignore;
mod launch;

// use launch;

fn main() {
    let args = env::args().skip(1);
    let len = args.len();

    if len == 0 {
        launch::find_todos();
    } else if len == 1 {
        println!("placeholder");
    } else {
        println!("placeholder");
    }
}
