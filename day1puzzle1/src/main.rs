use regex::Regex;
use std::io;

fn main() {
    let mut total = 0;
    for line in io::stdin().lines() {
        total = total
            + match line {
                Ok(msg) => read_num(msg),
                Err(_) => break,
            }
    }
    println!("total: {total}")
}

fn read_num(s: String) -> u32 {
    let find_num: Regex = Regex::new(r"[0-9]").unwrap();
    let numbers: Vec<&str> = find_num.find_iter(&s).map(|m| m.as_str()).collect();
    return match (numbers.first(), numbers.last()) {
        (Some(first), Some(last)) => format!("{first}{last}").parse().unwrap(),
        _ => 0,
    };
}
