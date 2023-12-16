use regex::Regex;
use std::collections::HashMap;
use std::io;
struct Schematic(Vec<String>);

impl Schematic {
    fn new(lines: impl Iterator<Item = String>) -> Schematic {
        Schematic(Vec::from_iter(lines))
    }

    fn get(&self, row: usize, col: usize) -> Option<u8> {
        let row = self.0.get(row)?.as_bytes();
        if row.len() <= col {
            return None;
        }
        Some(row[col])
    }
}

struct PartIter<'a> {
    schematic: &'a Schematic,
    row: usize,
    col: usize,
    find_num: Regex,
}

struct Part<'a> {
    schematic: &'a Schematic,
    row: usize,
    start: usize,
    end: usize,
    val: usize,
}

impl PartIter<'_> {
    fn new(schematic: &Schematic) -> PartIter {
        let find_num = Regex::new("[0-9]+").unwrap();
        PartIter {
            schematic,
            row: 0,
            col: 0,
            find_num,
        }
    }
}

impl<'a> Iterator for PartIter<'a> {
    type Item = Part<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut num_match = None;
        while num_match.is_none() {
            num_match = match self.schematic.0.get(self.row) {
                Some(line) => self.find_num.find_at(line, self.col),
                None => {
                    return None;
                }
            };
            if num_match.is_none() {
                self.row += 1;
                self.col = 0
            }
        }

        let num_match = num_match.unwrap();
        self.col = num_match.end();
        Some(Part {
            schematic: self.schematic,
            row: self.row,
            start: num_match.start(),
            end: num_match.end(),
            val: num_match.as_str().parse().unwrap(),
        })
    }
}

enum Gear {
    Incomplete(usize),
    Complete(usize, usize),
    OverLimit,
}

impl Gear {
    fn include(&self, num: usize) -> Gear {
        match self {
            Self::Incomplete(first) => Self::Complete(first.clone(), num),
            _ => Self::OverLimit,
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Coordinate {
    row: usize,
    col: usize,
}

fn main() {
    let mut total = 0;
    let schematic = Schematic::new(io::stdin().lines().filter_map(|line| match line {
        Ok(l) => Some(l),
        Err(_) => None,
    }));
    for part in PartIter::new(&schematic).filter_map(|p| {
        let mut row_start = p.row;
        if row_start > 0 {
            row_start -= 1;
        }
        for r in row_start..(p.row + 2) {
            let mut col_start = p.start;
            if col_start > 0 {
                col_start -= 1;
            }
            for c in col_start..(p.end + 1) {
                match p.schematic.get(r, c) {
                    Some(c) => {
                        if c != b'.' && (c < b'0' || c > b'9') {
                            return Some(p.val);
                        }
                    }
                    None => (),
                }
            }
        }
        None
    }) {
        total += part
    }
    println!("total puzzle 1: {total}");

    total = 0;
    let mut gears: HashMap<Coordinate, Gear> = HashMap::new();
    for p in PartIter::new(&schematic) {
        let mut row_start = p.row;
        if row_start > 0 {
            row_start -= 1;
        }
        for r in row_start..(p.row + 2) {
            let mut col_start = p.start;
            if col_start > 0 {
                col_start -= 1;
            }
            for c in col_start..(p.end + 1) {
                match p.schematic.get(r, c) {
                    Some(b'*') => {
                        let coor = Coordinate { row: r, col: c };
                        gears.insert(
                            coor,
                            match gears.get(&coor) {
                                Some(g) => g.include(p.val),
                                None => Gear::Incomplete(p.val),
                            },
                        );
                    }
                    _ => (),
                }
            }
        }
    }
    for (_, g) in &gears {
        match g {
            &Gear::Complete(first, second) => total += first * second,
            _ => (),
        }
    }
    println!("total puzzle 2: {total}");
}
