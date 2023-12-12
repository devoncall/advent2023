use std::slice::SliceIndex;

use regex::Regex;

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
    println!("Hello, world!");
}
