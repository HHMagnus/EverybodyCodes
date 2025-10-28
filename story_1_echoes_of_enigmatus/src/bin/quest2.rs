use std::collections::HashMap;
use std::fs::read_to_string;
use std::ops::Deref;

#[derive(Debug, Clone)]
struct Node {
    value: u64,
    letter: char,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

#[derive(Debug, Clone)]
struct NoteLine {
    id: u64,
    left: Node,
    right: Node,
}

fn main() {
    let file = read_to_string("input/quest2/input1.txt").unwrap();
    let input = file.lines()
        .map(|line| line.split(" ").collect::<Vec<_>>())
        .map(|line| {
            let left = line[2].replace("left=[", "")
                .replace("]", "");
            let mut left = left.split(",");
            let right = line[3].replace("right=[", "")
                .replace("]", "");
            let mut right = right.split(",");
            NoteLine::new(
                line[1].split("=").last().unwrap().parse::<u64>().unwrap(),
                left.next().unwrap().parse::<u64>().unwrap(),
                left.next().unwrap().chars().next().unwrap(),
                right.next().unwrap().parse::<u64>().unwrap(),
                right.next().unwrap().chars().next().unwrap(),
            )
        }).collect::<Vec<_>>();

    let mut iterator = input.into_iter();

    let first = iterator.next().unwrap();
    let mut left = first.left;
    let mut right = first.right;
    while let Some(element) = iterator.next() {
        left.insert(element.left);
        right.insert(element.right);
    }

    println!("{}{}", left.traverse(), right.traverse());
}

impl Node {
    fn new(value: u64, letter: char) -> Node {
        Node {
            value,
            letter,
            left: None,
            right: None,
        }
    }

    fn insert(&mut self, node: Node) {
        if node.value < self.value {
            if self.left.is_some() {
                self.left.as_mut().unwrap().insert(node);
            } else {
                self.left = Some(Box::new(node));
            }
        } else {
            if self.right.is_some() {
                self.right.as_mut().unwrap().insert(node);
            } else {
                self.right = Some(Box::new(node));
            }
        }
    }

    fn traverse(&self) -> String {
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
                if let Some(x) = element.right {
                    new_elements.push(x.deref().clone());
                }
                if let Some(x) = element.left {
                    new_elements.push(x.deref().clone());
                }
            }
            elements = new_elements;
            depth += 1;
        }

        let part1 = map.into_iter()
            .max_by_key(|(_k, v)| v.len());
        let binding = part1.unwrap();
        let mut part1 = binding
            .1
            .iter()
            .collect::<Vec<_>>();
        part1.sort_by_key(|n| n.value);
        part1.into_iter().map(|n| n.letter).collect()
    }
}

impl NoteLine {
    fn new(id: u64, left_num: u64, left_char: char, right_num: u64, right_char: char) -> Self {
        NoteLine {
            id,
            left: Node::new(left_num, left_char),
            right: Node::new(right_num, right_char),
        }
    }
}