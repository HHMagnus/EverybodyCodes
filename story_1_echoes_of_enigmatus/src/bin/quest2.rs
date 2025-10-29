use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;
use std::ops::Deref;

type Swaps = HashSet<u64>;

#[derive(Debug, Clone)]
struct Node {
    id: u64,
    is_left: bool,
    left_value: u64,
    left_letter: char,
    right_value: u64,
    right_letter: char,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

#[derive(Debug, Clone)]
struct Add {
    id: u64,
    left: Node,
    right: Node,
}

#[derive(Debug, Clone)]
struct Swap {
    id: u64,
}

#[derive(Debug, Clone)]
enum Instruction {
    Add(Add),
    Swap(Swap),
}

impl Instruction {
    fn parse(input: &str) -> Instruction {
        let line = input.split(" ").collect::<Vec<_>>();
        let instruction = line[0];
        if instruction == "SWAP" {
            let id = line[1].parse().unwrap();
            return Instruction::Swap(Swap { id });
        }
        let left = line[2].replace("left=[", "")
            .replace("]", "");
        let mut left = left.split(",");
        let right = line[3].replace("right=[", "")
            .replace("]", "");
        let mut right = right.split(",");
        let add = Add::new(
            line[1].split("=").last().unwrap().parse::<u64>().unwrap(),
            left.next().unwrap().parse::<u64>().unwrap(),
            left.next().unwrap().chars().next().unwrap(),
            right.next().unwrap().parse::<u64>().unwrap(),
            right.next().unwrap().chars().next().unwrap(),
        );
        Instruction::Add(add)
    }
}

fn main() {
    let file = read_to_string("input/quest2/input2.txt").unwrap();
    let input = file.lines()
        .map(|line| Instruction::parse(line))
        .collect::<Vec<_>>();

    let mut iterator = input.into_iter();

    let mut swaps = HashSet::new();

    let first = iterator.next().unwrap();
    let first = match first {
        Instruction::Add(add) => add,
        _ => unreachable!("First should always be Add variant"),
    };
    let mut left = first.left;
    let mut right = first.right;
    while let Some(element) = iterator.next() {
        match element {
            Instruction::Add(add) => {
                left.insert(add.left, &swaps);
                right.insert(add.right, &swaps);
            }
            Instruction::Swap(swap) => {
                if swaps.contains(&swap.id) {
                    swaps.remove(&swap.id);
                } else {
                    swaps.insert(swap.id);
                }
            }
        }
    }

    println!("{}{}", left.traverse(&swaps), right.traverse(&swaps));
}

impl Node {
    fn new(id: u64, is_left: bool, left_value: u64, left_letter: char, right_value: u64, right_letter: char) -> Node {
        Node {
            id,
            is_left,
            left_value,
            left_letter,
            right_value,
            right_letter,
            left: None,
            right: None,
        }
    }

    fn is_left(&self, swaps: &Swaps) -> bool {
        if swaps.contains(&self.id) {
            return !self.is_left;
        }
        self.is_left
    }

    fn get_value(&self, swaps: &Swaps) -> u64 {
        if self.is_left(swaps) {
            self.left_value
        } else {
            self.right_value
        }
    }

    fn get_letter(&self, swaps: &Swaps) -> char {
        if self.is_left(swaps) {
            self.left_letter
        } else {
            self.right_letter
        }
    }

    fn insert(&mut self, node: Node, swaps: &Swaps) {
        if node.get_value(swaps) < self.get_value(swaps) {
            if self.left.is_some() {
                self.left.as_mut().unwrap().insert(node, swaps);
            } else {
                self.left = Some(Box::new(node));
            }
        } else {
            if self.right.is_some() {
                self.right.as_mut().unwrap().insert(node, swaps);
            } else {
                self.right = Some(Box::new(node));
            }
        }
    }

    fn traverse(&self, swaps: &Swaps) -> String {
        let mut depth = 0;
        let mut elements = Vec::new();
        if let Some(x) = &self.left {
            elements.push(x.deref().clone());
        }
        if let Some(x) = &self.right {
            elements.push(x.deref().clone());
        }
        let mut map = HashMap::new();

        while !elements.is_empty() {
            map.insert(depth, elements.clone());
            let mut new_elements = Vec::new();
            for element in elements {
                if let Some(x) = element.left {
                    new_elements.push(x.deref().clone());
                }
                if let Some(x) = element.right {
                    new_elements.push(x.deref().clone());
                }
            }
            elements = new_elements;
            depth += 1;
        }

        map.into_iter()
            .max_by_key(|(_k, v)| v.len())
            .unwrap()
            .1
            .into_iter()
            .map(|n| n.get_letter(swaps)).collect()
    }
}

impl Add {
    fn new(id: u64, left_num: u64, left_char: char, right_num: u64, right_char: char) -> Self {
        Add {
            id,
            left: Node::new(id, true, left_num, left_char, right_num, right_char),
            right: Node::new(id, false, left_num, left_char, right_num, right_char),
        }
    }
}