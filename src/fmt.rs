pub fn fmt_blue<T>(s: &T) -> String 
where T: std::fmt::Display
{
    return format!("\x1b[34;1m{}\x1b[0m", s);
}

pub fn fmt_yellow<T>(s: &T) -> String 
where T: std::fmt::Display
{
    return format!("\x1b[33;1m{}\x1b[0m", s);
}

pub fn fmt_orange<T>(s: &T) -> String 
where T: std::fmt::Display
{
    return format!("\x1b[38;2;252;127;0m{}\x1b[0m", s);
}

pub fn fmt_red<T>(s: &T) -> String 
where T: std::fmt::Display 
{
    return format!("\x1b[31;1m{}\x1b[0m", s);
}

pub fn fmt_underline(s: &str) -> String {
    return format!("\x1b[;1;4m{}\x1b[0m", s);
}
