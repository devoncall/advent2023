use regex::Regex;
use std::collections::HashMap;
use std::io;

struct Node {
    label: [u8; 3],
    left: [u8; 3],
    right: [u8; 3],
}

const START_NODE: [u8; 3] = [b'A', b'A', b'A'];
const END_NODE: [u8; 3] = [b'Z', b'Z', b'Z'];

struct NodeMap {
    nodes: HashMap<[u8; 3], Node>,
}

enum Direction {
    Left,
    Right,
}

impl NodeMap {
    fn traverse<'a, 'b, 'c>(
        &'a self,
        start: &'a [u8],
        path: &'b [Direction],
        step_counter: &'c mut usize,
    ) -> &'a [u8] {
        if path.len() == 0 {
            return start;
        }
        let mut current = self.nodes.get(start).unwrap();
        for d in path {
            current = match d {
                Direction::Left => self.nodes.get(current.left.as_ref()).unwrap(),
                Direction::Right => self.nodes.get(current.right.as_ref()).unwrap(),
            };
            *step_counter += 1;
        }
        return &current.label;
    }
}

fn main() {
    let mut lines = io::stdin().lines();
    let path: Vec<Direction> = lines
        .next()
        .expect("missing first line")
        .unwrap()
        .as_bytes()
        .iter()
        .filter_map(|c| match c {
            b'R' => Some(Direction::Right),
            b'L' => Some(Direction::Left),
            _ => None,
        })
        .collect();
    let node_pattern = Regex::new(r"([A-Z]{3}) = \(([A-Z]{3}), ([A-Z]{3})\)").unwrap();
    lines.next();
    let mut node_map = NodeMap {
        nodes: HashMap::new(),
    };
    for line in lines.filter_map(|l| match l {
        Ok(s) => Some(s),
        Err(_) => None,
    }) {
        let capture = node_pattern.captures(line.as_str()).unwrap();
        let (_, [label_str, left_str, right_str]) = capture.extract();

        let label_bytes = label_str.as_bytes();
        let label = [label_bytes[0], label_bytes[1], label_bytes[2]];

        let left_bytes = left_str.as_bytes();
        let left = [left_bytes[0], left_bytes[1], left_bytes[2]];

        let right_bytes = right_str.as_bytes();
        let right = [right_bytes[0], right_bytes[1], right_bytes[2]];

        node_map.nodes.insert(label, Node { label, left, right });
    }

    let mut steps = 0;
    let mut current = START_NODE.as_ref();
    while current != END_NODE {
        current = node_map.traverse(current, path.as_slice(), &mut steps);
    }
    println!("puzzle 1: {steps}");
}
