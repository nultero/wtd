use std::env;
mod dir_search;

fn main() {

    let args = env::args().skip(1);
    let len = args.len();

    if len == 0 {
        dir_search::search();

    } else if len == 1 {
        println!("placeholder");
        
    } else {
        println!("placeholder");
    }
}