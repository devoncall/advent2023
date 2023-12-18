use std::cmp::Ordering;
use std::io;

#[derive(PartialEq, Eq, Clone, Copy)]
struct Hand {
    cards: [u8; 5],
}

impl Hand {
    fn card_index(card: u8) -> Option<usize> {
        match card {
            b'2' => Some(0),
            b'3' => Some(1),
            b'4' => Some(2),
            b'5' => Some(3),
            b'6' => Some(4),
            b'7' => Some(5),
            b'8' => Some(6),
            b'9' => Some(7),
            b'T' => Some(8),
            b'J' => Some(9),
            b'Q' => Some(10),
            b'K' => Some(11),
            b'A' => Some(12),
            _ => None,
        }
    }

    fn counts(&self) -> [u8; 5] {
        *self
            .cards
            .iter()
            .filter_map(|card| Hand::card_index(*card))
            .fold(&mut [0; 13], |counter, index| {
                counter[index] += 1;
                counter
            })
            .iter()
            .fold(&mut [0u8; 5], |cts, c| match c {
                0 => cts,
                _ => {
                    cts[*c - 1] += 1;
                    cts
                }
            })
    }

    fn get_type(&self) -> HandType {
        match self.counts() {
            [0, 0, 0, 0, 1] => HandType::FiveSet,
            [1, 0, 0, 1, 0] => HandType::FourSet,
            [0, 1, 1, 0, 0] => HandType::FullHouse,
            [2, 0, 1, 0, 0] => HandType::ThreeSet,
            [1, 2, 0, 0, 0] => HandType::TwoPair,
            [3, 1, 0, 0, 0] => HandType::Pair,
            _ => HandType::HighCard,
        }
    }

    fn to_wildj(&self) -> HandWildJ {
        HandWildJ { cards: self.cards }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let (left_type, right_type) = (self.get_type(), other.get_type());
        match left_type.cmp(&right_type) {
            Ordering::Less => return Some(Ordering::Less),
            Ordering::Greater => return Some(Ordering::Greater),
            _ => Some(Ordering::Equal),
        };
        for index in 0..5 {
            let order = match (
                Hand::card_index(self.cards[index]),
                Hand::card_index(other.cards[index]),
            ) {
                (Some(left), Some(right)) => left.partial_cmp(&right),
                _ => None,
            };
            if order.is_none() || order.is_some_and(|o| o != Ordering::Equal) {
                return order;
            }
        }
        Some(Ordering::Equal)
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.partial_cmp(other) {
            Some(o) => o,
            None => Ordering::Less,
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    FiveSet = 7,
    FourSet = 6,
    FullHouse = 5,
    ThreeSet = 4,
    TwoPair = 3,
    Pair = 2,
    HighCard = 1,
}

#[derive(PartialEq, Eq, Clone, Copy)]
struct HandWildJ {
    cards: [u8; 5],
}
impl HandWildJ {
    fn card_index(card: u8) -> Option<usize> {
        match card {
            b'J' => Some(0),
            b'2' => Some(1),
            b'3' => Some(2),
            b'4' => Some(3),
            b'5' => Some(4),
            b'6' => Some(5),
            b'7' => Some(6),
            b'8' => Some(7),
            b'9' => Some(8),
            b'T' => Some(9),
            b'Q' => Some(10),
            b'K' => Some(11),
            b'A' => Some(12),
            _ => None,
        }
    }

    fn to_hand(&self) -> Hand {
        Hand {
            cards: [
                self.cards[0],
                self.cards[1],
                self.cards[2],
                self.cards[3],
                self.cards[4],
            ],
        }
    }

    fn replace_j(&self, card: u8) -> Hand {
        let mut result = Hand { cards: self.cards };
        for i in 0..5 {
            if result.cards[i] == b'J' {
                result.cards[i] = card;
            }
        }
        result
    }

    fn get_type(&self) -> HandType {
        if !self.cards.contains(&b'J') {
            return self.to_hand().get_type();
        }
        [
            b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9', b'T', b'Q', b'K', b'A',
        ]
        .iter()
        .map(|card| self.replace_j(*card).get_type())
        .max()
        .unwrap_or(HandType::HighCard)
    }
}

impl PartialOrd for HandWildJ {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let (left_type, right_type) = (self.get_type(), other.get_type());
        match left_type.cmp(&right_type) {
            Ordering::Less => return Some(Ordering::Less),
            Ordering::Greater => return Some(Ordering::Greater),
            _ => Some(Ordering::Equal),
        };
        for index in 0..5 {
            let order = match (
                HandWildJ::card_index(self.cards[index]),
                HandWildJ::card_index(other.cards[index]),
            ) {
                (Some(left), Some(right)) => left.partial_cmp(&right),
                _ => None,
            };
            if order.is_none() || order.is_some_and(|o| o != Ordering::Equal) {
                return order;
            }
        }
        Some(Ordering::Equal)
    }
}

impl Ord for HandWildJ {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.partial_cmp(other) {
            Some(o) => o,
            None => Ordering::Less,
        }
    }
}

fn main() {
    let mut hands: Vec<(Hand, usize)> = io::stdin()
        .lines()
        .filter_map(|line| match line {
            Ok(l) => Some(l),
            Err(_) => None,
        })
        .filter_map(|s| match s.split_once(' ') {
            Some((hand_str, bid_str)) => {
                let hand_slice = hand_str.as_bytes();
                if hand_slice.len() != 5 {
                    return None;
                }
                Some((
                    Hand {
                        cards: [
                            hand_slice[0],
                            hand_slice[1],
                            hand_slice[2],
                            hand_slice[3],
                            hand_slice[4],
                        ],
                    },
                    bid_str.parse::<usize>().unwrap(),
                ))
            }
            _ => None,
        })
        .collect();
    hands.sort_unstable_by_key(|(hand, _)| *hand);
    let total = hands
        .iter()
        .enumerate()
        .map(|v| (v.0 + 1, v.1))
        .fold(0, |acc, row| acc + row.0 * row.1 .1);
    println!("puzzle 1 total: {total}");

    let mut hands_wildj: Vec<(HandWildJ, usize)> = hands
        .iter()
        .map(|(hand, bid)| (hand.to_wildj(), *bid))
        .collect();
    hands_wildj.sort_unstable_by_key(|(hand, _)| *hand);
    let total = hands_wildj
        .iter()
        .enumerate()
        .map(|v| (v.0 + 1, v.1))
        .fold(0, |acc, row| acc + row.0 * row.1 .1);
    println!("puzzle 2 total: {total}")
}
