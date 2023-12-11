use regex::Regex;
use std::cmp::max;
use std::io;

#[derive(Debug)]
struct Game {
    id: u32,
    clues: Vec<Clue>,
}

impl Game {
    fn new(line: &str) -> Option<Game> {
        let num_re = Regex::new(r"\d+").unwrap();
        let parts = line.split_once(':')?;
        let id: u32 = num_re.find(parts.0)?.as_str().parse().unwrap();
        let clues_iter = parts.1.split(';').map(|clue| Clue::new(clue));
        Some(Game {
            id,
            clues: clues_iter.collect(),
        })
    }

    fn possible(&self, limit: Clue) -> bool {
        self.clues.iter().all(|clue| {
            clue.red <= limit.red && clue.green <= limit.green && clue.blue <= limit.blue
        })
    }

    fn minimum(&self) -> Clue {
        self.clues
            .iter()
            .copied()
            .reduce(|total, clue| Clue {
                red: max(total.red, clue.red),
                green: max(total.green, clue.green),
                blue: max(total.blue, clue.blue),
            })
            .unwrap_or_default()
    }
}

#[derive(Debug, Default, Clone, Copy)]
struct Clue {
    red: u32,
    green: u32,
    blue: u32,
}

enum Color {
    Blue,
    Red,
    Green,
}

impl Clue {
    fn new(data: &str) -> Clue {
        let clue_re = Regex::new(r"(\d+)\s+(red|blue|green)").unwrap();
        data.split(',')
            .map(|part| {
                let clue_parts = clue_re.captures(part)?;
                match (clue_parts.get(1), clue_parts.get(2)) {
                    (Some(num), Some(color)) => Some((
                        num.as_str().parse().unwrap(),
                        Color::from_str(color.as_str())?,
                    )),
                    _ => None,
                }
            })
            .fold(
                Clue {
                    red: 0,
                    green: 0,
                    blue: 0,
                },
                |clue, update| match update {
                    Some((num, Color::Red)) => Clue {
                        red: num,
                        green: clue.green,
                        blue: clue.blue,
                    },
                    Some((num, Color::Blue)) => Clue {
                        red: clue.red,
                        green: clue.green,
                        blue: num,
                    },
                    Some((num, Color::Green)) => Clue {
                        red: clue.red,
                        green: num,
                        blue: clue.blue,
                    },
                    _ => clue,
                },
            )
    }

    fn power(&self) -> u32 {
        self.red * self.green * self.blue
    }
}

impl Color {
    fn from_str(name: &str) -> Option<Color> {
        match name {
            "red" => Some(Color::Red),
            "green" => Some(Color::Green),
            "blue" => Some(Color::Blue),
            _ => None,
        }
    }
}

fn main() {
    let mut total = 0;
    for line in io::stdin().lines() {
        total = total
            + match line {
                Ok(msg) => {
                    let game = Game::new(msg.as_str());
                    match game {
                        Some(g) => g.minimum().power(),
                        None => 0,
                    }
                }
                Err(_) => break,
            }
    }
    println!("total: {total}")
}
