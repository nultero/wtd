use std::env;

mod dir_search;
mod flags;
mod fmt;
mod ignore;
mod launch;
mod search;

fn main() {
    let args = env::args().skip(1);

    let len = args.len();
    if len == 0 {
        launch::find_todos(flags::standard_flags());
    } else {
        println!("placeholder");
    }
}
