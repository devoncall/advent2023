use regex::Regex;
use std::io;

fn winning_range(time: f64, record: f64) -> (u64, u64) {
    let margin = (time * time - 4f64 * record).sqrt();
    let mut lower = (time - margin) / 2.0;
    let mut upper = (time + margin) / 2.0;
    if lower.ceil() == lower {
        lower += 1.0;
    }
    if upper.floor() == upper {
        upper -= 1.0;
    }
    return (lower.ceil() as u64, upper.floor() as u64);
}

fn main() {
    let mut input = io::stdin().lines();
    let time_line = input.next().unwrap().unwrap();
    let times: Vec<f64> = time_line
        .split(' ')
        .filter_map(|s| match s.parse() {
            Ok(n) => Some(n),
            _ => None,
        })
        .collect();
    let record_line = input.next().unwrap().unwrap();
    let records: Vec<f64> = record_line
        .split(' ')
        .filter_map(|s| match s.parse() {
            Ok(n) => Some(n),
            _ => None,
        })
        .collect();
    let solution1 = (0..times.len())
        .map(|index| {
            // let time = times[index];
            // let record = records[index];
            // print!("time {time}, record {record}, ");
            winning_range(times[index], records[index])
        })
        .map(|pair| {
            // let (left, right) = pair;
            // print!("winning: {left}, {right}, ");
            (pair.1 - pair.0) + 1
        })
        .reduce(|acc, val| {
            // println!("possible {val}");
            acc * val
        })
        .unwrap();
    println!("puzzle 1: {solution1}");
    let is_num = Regex::new(r"[0-9]+").unwrap();
    let time: f64 = time_line
        .split(' ')
        .filter(|s| is_num.is_match(s))
        .fold(String::new(), |mut acc, s| {
            acc.push_str(s);
            acc
        })
        .parse()
        .unwrap();
    let record: f64 = record_line
        .split(' ')
        .filter(|s| is_num.is_match(s))
        .fold(String::new(), |mut acc, s| {
            acc.push_str(s);
            acc
        })
        .parse()
        .unwrap();
    let (left, right) = winning_range(time, record);
    let possible = (right - left) + 1;
    println!("puzzle 2: {possible}")
}
