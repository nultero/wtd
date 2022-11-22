use std::env;

mod flags;
mod fmt;
mod ignore;
mod launch;
mod matches;
mod search;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.len() == 0 {
        launch::find_todos(flags::standard_flags());
        
    } else { // only need to parse some flags, don't really need clap / phosisticated parser
        let flags = parse_flags(args);
        if !flags.help {
            launch::find_todos(flags);
        } else {
            print_help();
        }
    }
}

fn parse_flags(args: Vec<String>) -> flags::Flags {

    let helps: [String; 5] = [ 
        "h".to_string(), 
        "-h".to_string(), 
        "help".to_string(), 
        "-help".to_string(), 
        "--help".to_string(),
    ];

    let mut flagset = flags::standard_flags();
    for arg in &args {

        if helps.contains(arg) {
            flagset.help = true;
            break;
        }

        let chars: Vec<char> = arg.chars().collect();
        if chars[0] == '-' {
            for c in &chars[1..] {
                match c {
                'n' => { flagset.nostrip = true }
                'r' => { flagset.reverse = true }
                's' => { flagset.short   = true }
                _   => { unrecog_arg(c) }
                }
            }
        } else {
            unrecog_arg(arg);
        }
    }
    return flagset;
}

fn unrecog_arg<T: std::fmt::Display>(arg: T) {
    println!(
        "{} arg '{}' unrecognized",
        fmt::fmt_red(&"!!"),
        fmt::fmt_red(&arg),
    );
}

fn print_help() {
    let wtf = "wtd".to_string();
    let bluewtf = fmt::fmt_blue(&wtf);
    println!("{} -- {}hat {}o {}o", 
        bluewtf,
        fmt::fmt_blue(&'w'), 
        fmt::fmt_blue(&'t'), 
        fmt::fmt_blue(&'d'), 
    );

    println!("usage: wtd [OPTION]");

    let t = fmt::fmt_orange(&"TODO");
    let o = fmt::fmt_orange(&"O");

    println!("searches for '{}'s in files", t);
    println!(" (each {} having a priority based on its number of {}s)", t, o);

    let n = fmt::fmt_blue(&"-n");
    let r = fmt::fmt_blue(&"-r");
    let v = fmt::fmt_blue(&"-v");
    let vv = fmt::fmt_blue(&"-vv");
    let vvv = fmt::fmt_blue(&"-vvv");

    println!("\noptions:");

    println!("   {}    nostrip:", n);
    println!("          only works with at least 1 {} flag", v);
    println!("          normally {} trims the leftside / comment brackets", bluewtf);
    println!("          of {}s; this flag prevents that ", t);
    
    println!();
    
    println!("   {}    reverse:", r);
    println!("          inverts the priority of the {} sort", o);
    
    println!();

    let line_num = fmt::fmt_underline("line number");
    let ip = fmt::fmt_underline("individual priorities");
    let eln = fmt::fmt_underline("entire line");
    
    println!("   {},{},{}    verbose:", v, vv, vvv);
    println!("     {}     prints the {} of each {}", v, line_num, t);
    println!("     {}    prints the {} of each {}", vv, ip, t);
    println!("     {}   prints the {} of each {}", vvv, eln, t);
}