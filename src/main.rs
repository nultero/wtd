use std::env;

mod flags;
mod fmt;
mod ignore;
mod launch;
mod matches;
mod search;

fn main() {
    let args = env::args().skip(1);

    if args.len() == 0 {
        launch::find_todos(flags::standard_flags());
        
    } else { // only need to parse some flags, don't really need clap
        println!("placeholder");
    }
}
