use std::collections::HashSet;
use std::io;

struct Card {
    id: usize,
    target: HashSet<usize>,
    contains: Vec<usize>,
}

impl Card {
    fn new(line: String) -> Card {
        let (front, nums) = match line.split_once(':') {
            Some((f, n)) => (f, n),
            None => panic!("missing ':' delimiter"),
        };
        let mut result = Card {
            id: 0,
            target: HashSet::new(),
            contains: Vec::new(),
        };
        match front.split_once(' ') {
            Some((_, id_str)) => result.id = id_str.parse().unwrap_or(0),
            _ => (),
        };
        let (targets, has) = match nums.split_once('|') {
            Some((t, h)) => (t, h),
            None => panic!("missing '|' delimiter"),
        };
        for t in targets.split(' ') {
            if t.len() > 0 {
                result.target.insert(t.parse().unwrap());
            }
        }
        for h in has.split(' ') {
            if h.len() > 0 {
                result.contains.push(h.parse().unwrap());
            }
        }
        result
    }

    fn points(&self) -> usize {
        let matches = self
            .contains
            .iter()
            .filter(|num| self.target.contains(num))
            .count();
        match matches {
            0 => 0,
            _ => 2_usize.pow((matches - 1).try_into().unwrap()),
        }
    }

    fn matches(&self) -> usize {
        self.contains
            .iter()
            .filter(|num| self.target.contains(num))
            .count()
    }
}

fn main() {
    let cards: Vec<Card> = io::stdin()
        .lines()
        .filter_map(|line| match line {
            Ok(l) => Some(Card::new(l)),
            _ => None,
        })
        .collect();
    let total = cards.iter().fold(0, |t, card| t + card.points());
    println!("puzzle 1 total: {total}");

    let mut copies = vec![1; cards.len()];
    for indx in 0..cards.len() {
        let score = cards[indx].matches();
        if score > 0 {
            for delta in 1..score + 1 {
                copies[indx + delta] += copies[indx]
            }
        }
    }
    let total = copies.iter().fold(0, |t, copy| t + copy);
    println!("puzzle 2 total: {total}")
}
