use regex::Regex;
use std::io;

struct AlmanacMap {
    start: isize,
    end: isize,
    delta: isize,
}

impl AlmanacMap {
    fn new(start: isize, target: isize, range: isize) -> AlmanacMap {
        AlmanacMap {
            start,
            end: start + range,
            delta: target - start,
        }
    }
}

struct AlmanacSection {
    a_maps: Vec<AlmanacMap>,
}

impl AlmanacSection {
    fn apply(&self, val: isize) -> isize {
        for m in self.a_maps.iter() {
            if m.start <= val && m.end > val {
                return val + m.delta;
            }
        }
        return val;
    }
}

struct Almanac {
    sections: Vec<AlmanacSection>,
}

impl Almanac {
    fn apply(&self, seed: isize) -> isize {
        //print!("{seed}");
        let location = self.sections.iter().fold(seed, |val, sec| {
            let next = sec.apply(val);
            //print!("->{next}");
            next
        });
        //println!();
        return location;
    }
}

fn main() {
    let num_match = Regex::new(r"[0-9]+").unwrap();
    let mapping_line = Regex::new(r"([0-9]+) ([0-9]+) ([0-9]+)").unwrap();
    let mut lines = io::stdin().lines();
    let first = lines.next().unwrap().expect("missing seeds");
    let seeds: Vec<isize> = first
        .split(' ')
        .filter_map(|s| {
            if num_match.is_match(s) {
                return Some(s.parse::<isize>().unwrap());
            }
            None
        })
        .collect();
    let mut almanac: Almanac = Almanac {
        sections: Vec::<AlmanacSection>::new(),
    };
    almanac.sections.push(AlmanacSection {
        a_maps: Vec::<AlmanacMap>::new(),
    });
    let mut new_section_started = true;
    for line in lines.filter_map(|r_line| match r_line {
        Ok(line) => Some(line),
        Err(_) => None,
    }) {
        if mapping_line.is_match(line.as_str()) {
            new_section_started = false;
            let nums: Vec<isize> = line.split(' ').map(|s| s.parse().unwrap()).collect();
            almanac
                .sections
                .last_mut()
                .unwrap()
                .a_maps
                .push(AlmanacMap::new(nums[1], nums[0], nums[2]));
        } else if !new_section_started {
            almanac.sections.push(AlmanacSection {
                a_maps: Vec::<AlmanacMap>::new(),
            });
            new_section_started = true;
        }
    }
    let min_location = seeds.iter().map(|seed| almanac.apply(*seed)).min().unwrap();
    println!("min location, puzzle 1: {min_location}");
    let min_location_2 = (0..seeds.len())
        .step_by(2)
        .flat_map(|index| (seeds[index]..seeds[index] + seeds[index + 1]))
        .map(|seed| almanac.apply(seed))
        .min()
        .unwrap();
    println!("min location, puzzle 2: {min_location_2}");
}
