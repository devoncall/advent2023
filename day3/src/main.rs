use regex::Regex;
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
    println!("total: {total}")
}
