use regex::Regex;
use std::io;

fn main() {
    let mut total = 0;
    for line in io::stdin().lines() {
        total = total
            + match line {
                Ok(msg) => {
                    let num = read_num(msg.clone());
                    println!("{msg}: {num}");
                    num
                }
                Err(_) => break,
            }
    }
    println!("total: {total}")
}

fn read_num(s: String) -> u32 {
    let find_num: Regex =
        Regex::new(r"(?m)(?=([0-9]|one|two|three|four|five|six|seven|eight|nine))").unwrap();

    let numbers: Vec<&str> = find_num
        .captures_iter(&s)
        .map(|m| match m.get(1) {
            Some(number) => number.as_str(),
            None => "",
        })
        .collect();
    return match (numbers.first(), numbers.last()) {
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
