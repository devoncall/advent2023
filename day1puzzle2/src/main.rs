use regex::Regex;
use std::io;

fn main() {
    let mut total = 0;
    for line in io::stdin().lines() {
        total = total
            + match line {
                Ok(msg) => {
                    let num = read_num(msg.as_str());
                    println!("{msg}: {num}");
                    num
                }
                Err(_) => break,
            }
    }
    println!("total: {total}")
}

fn read_num(s: &str) -> u32 {
    let find_num: Regex =
        Regex::new(r"([0-9]|one|two|three|four|five|six|seven|eight|nine)").unwrap();
    let mut index = 0;
    let mut first: Option<&str> = None;
    let mut last: Option<&str> = None;
    while index < s.len() {
        match find_num.find_at(s, index) {
            Some(found) => {
                if first.is_none() {
                    first = Some(found.as_str());
                }
                last = Some(found.as_str());
                index = found.start() + 1;
            }
            None => break,
        }
    }
    return match (first, last) {
        (Some(first), Some(last)) => {
            let f = to_digit(first);
            let l = to_digit(last);
            format!("{f}{l}").parse().unwrap()
        }
        _ => 0,
    };
}

fn to_digit(s: &str) -> &str {
    return match s {
        "one" => "1",
        "two" => "2",
        "three" => "3",
        "four" => "4",
        "five" => "5",
        "six" => "6",
        "seven" => "7",
        "eight" => "8",
        "nine" => "9",
        _ => s,
    };
}
