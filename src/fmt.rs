/*

"darkblue": "\x1b[34;1;4m%v\x1b[0m",
"blue":     "\x1b[34;1m%v\x1b[0m",
"bold":     "\x1b[;1;1m%v\x1b[0m",
"cyan":     "\x1b[36;1;1m%v\x1b[0m",
"green":    "\x1b[32;1m%v\x1b[0m",
"magenta":  "\x1b[35;1m%v\x1b[0m",
"orange":   "\x1b[38;2;252;127;0m%v\x1b[0m",
"pink":     "\x1b[35;1;1m%v\x1b[0m",
"red":      "\x1b[31;1;1m%v\x1b[0m",
"uline":    "\x1b[;1;4m%v\x1b[0m",
"yellow":   "\x1b[33;1m%v\x1b[0m",

*/

pub fn fmt_blue(s: &str) -> String {
    return format!("\x1b[34;1m{}\x1b[0m", s);
}

pub fn fmt_yellow(s: &str) -> String {
    return format!("\x1b[33;1m{}\x1b[0m", s);
}

pub fn fmt_orange(s: &str) -> String {
    return format!("\x1b[38;2;252;127;0m{}\x1b[0m", s);
}

pub fn fmt_red(s: &str) -> String {
    return format!("\x1b[31;1m{}\x1b[0m", s);
}

pub fn fmt_underline(s: &str) -> String {
    return format!("\x1b[;1;4m{}\x1b[0m", s);
}
