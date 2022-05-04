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

        let mut valid = false;

        if helps.contains(arg) {
            flagset.help = true;
            valid = true;
        }

        if arg.contains("-v") && flagset.verbose != 3 {
            flagset.verbose = count_v(arg);
            valid = true;
        }

        if arg.contains("-n") {
            flagset.nostrip = true;
            valid = true;
        }

        if arg.contains("-r") {
            flagset.reverse = true;
            valid = true;
        }

        if !valid {
            println!(
                "{} arg '{}' unrecognized",
                fmt::fmt_red("!!"),
                fmt::fmt_red(arg),
            );
        }
    }

    return flagset;
}

#[allow(non_upper_case_globals)]
fn count_v(arg: &String) -> i8 {
    let mut i: i8 = 0;
    const v: u8 = 118;
    for byte in arg.as_bytes() {
        if byte == &v && i < 3 {
            i += 1;
        }
    }
    return i;
}

fn print_help() {
    let wtf = "wtf".to_string();
    let bluewtf = fmt::fmt_blue(&wtf);
    println!("{} -- {}here {}o {}ix", 
        bluewtf,
        fmt::fmt_blue(&wtf.chars().nth(0).unwrap().to_string()), 
        fmt::fmt_blue(&wtf.chars().nth(1).unwrap().to_string()), 
        fmt::fmt_blue(&wtf.chars().nth(2).unwrap().to_string()), 
    );

    println!("usage: wtf [OPTION]");

    let t = fmt::fmt_orange("TODO");
    let o = fmt::fmt_orange("O");

    println!("searches for '{}'s in files", t);
    println!(" (each {} having a priority based on its number of {}s)", t, o);

    let n = fmt::fmt_blue("-n");
    let r = fmt::fmt_blue("-r");
    let v = fmt::fmt_blue("-v");
    let vv = fmt::fmt_blue("-vv");
    let vvv = fmt::fmt_blue("-vvv");

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