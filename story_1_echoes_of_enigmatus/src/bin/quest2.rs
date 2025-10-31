use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;
use std::rc::Rc;

struct Swaps {
    set: HashSet<u64>,
    move_tree: bool,
}

#[derive(Debug, Clone)]
struct Node {
    id: u64,
    is_left: bool,
    left_value: u64,
    left_letter: char,
    right_value: u64,
    right_letter: char,
    partner: Option<Rc<RefCell<Node>>>,
    left: Option<Rc<RefCell<Node>>>,
    right: Option<Rc<RefCell<Node>>>,
}

#[derive(Debug, Clone)]
struct Add {
    id: u64,
    left: Rc<RefCell<Node>>,
    right: Rc<RefCell<Node>>,
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
    let file = read_to_string("input/quest2/input1.txt").unwrap();
    let part1 = solve(file, false);
    println!("Part 1: {}", part1);

    let file = read_to_string("input/quest2/input2.txt").unwrap();
    let part2 = solve(file, false);
    println!("Part 2: {}", part2);

    let file = read_to_string("input/quest2/input3.txt").unwrap();
    let part3 = solve(file, true);
    println!("Part 3: {}", part3);

}

fn solve(file: String, move_tree: bool) -> String {
    let input = file.lines()
        .map(|line| Instruction::parse(line))
        .collect::<Vec<_>>();

    let mut iterator = input.into_iter();

    let mut swaps = Swaps {
        set: HashSet::new(),
        move_tree,
    };

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
                left.borrow_mut().insert(add.left, &swaps);
                right.borrow_mut().insert(add.right, &swaps);
            }
            Instruction::Swap(swap) => {
                if move_tree && swap.id == first.id {
                    let temp = left;
                    left = right;
                    right = temp;
                    continue;
                }
                swaps.swap(&swap.id);
            }
        }
    }

    format!("{}{}", left.borrow().traverse(&swaps), right.borrow().traverse(&swaps))
}

impl Swaps {
    fn swap(&mut self, id: &u64) {
        if self.set.contains(&id) {
            self.set.remove(&id);
        } else {
            self.set.insert(id.clone());
        }
    }

    fn should_switch_value(&self, id: &u64) -> bool {
        if (self.move_tree) {
            false
        } else {
            self.set.contains(id)
        }
    }

    fn should_switch_node(&self, id: &u64) -> bool {
        if (self.move_tree) {
            self.set.contains(id)
        } else {
            false
        }
    }
}
impl Node {
    fn new(id: u64, left_value: u64, left_letter: char, right_value: u64, right_letter: char)
        -> (Rc<RefCell<Node>>, Rc<RefCell<Node>>) {
        let left_node = Node {
            id,
            is_left: true,
            left_value,
            left_letter,
            right_value,
            right_letter,
            partner: None,
            left: None,
            right: None,
        };
        let left_node = Rc::new(RefCell::new(left_node));
        let right_node = Node {
            id,
            is_left: false,
            left_value,
            left_letter,
            right_value,
            right_letter,
            partner: None,
            left: None,
            right: None,
        };
        let right_node = Rc::new(RefCell::new(right_node));
        left_node.borrow_mut().set_partner(right_node.clone());
        right_node.borrow_mut().set_partner(left_node.clone());
        (left_node, right_node)
    }

    fn set_partner(&mut self, partner: Rc<RefCell<Node>>) {
        self.partner = Some(partner);
    }

    fn is_left(&self, swaps: &Swaps) -> bool {
        if swaps.should_switch_value(&self.id) {
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

    fn left(&self, swaps: &Swaps) -> Rc<RefCell<Node>> {
        let actual_left = self.left.clone().unwrap().clone();
        if swaps.should_switch_node(&actual_left.borrow().id) {
            actual_left.borrow().partner.clone().unwrap().clone()
        } else {
            actual_left
        }
    }

    fn right(&self, swaps: &Swaps) -> Rc<RefCell<Node>> {
        let actual_right = self.right.clone().unwrap().clone();
        if swaps.should_switch_node(&actual_right.borrow().id) {
            actual_right.borrow().partner.clone().unwrap().clone()
        } else {
            actual_right
        }
    }

    fn insert(&mut self, node: Rc<RefCell<Node>>, swaps: &Swaps) {
        if node.borrow().get_value(swaps) < self.get_value(swaps) {
            if self.left.is_some() {
                self.left(swaps).borrow_mut().insert(node, swaps);
            } else {
                self.left = Some(node.clone());
            }
        } else {
            if self.right.is_some() {
                self.right(swaps).borrow_mut().insert(node, swaps);
            } else {
                self.right = Some(node.clone());
            }
        }
    }

    fn traverse(&self, swaps: &Swaps) -> String {
        let mut depth = 0;
        let mut current_layer_elements = Vec::new();
        if self.left.is_some() {
            current_layer_elements.push(self.left(swaps));
        }
        if self.right.is_some() {
            current_layer_elements.push(self.right(swaps));
        }
        let mut depth_to_elements = HashMap::new();

        while !current_layer_elements.is_empty() {
            depth_to_elements.insert(depth, current_layer_elements.clone());
            let mut new_elements = Vec::new();
            for element in current_layer_elements {
                if element.borrow().left.is_some() {
                    new_elements.push(element.borrow().left(swaps));
                }
                if element.borrow().right.is_some() {
                    new_elements.push(element.borrow().right(swaps));
                }
            }
            current_layer_elements = new_elements;
            depth += 1;
        }

        depth_to_elements.into_iter()
            .max_by_key(|(k, v)| (v.len(), -k))
            .unwrap()
            .1
            .into_iter()
            .map(|n| n.borrow().get_letter(swaps)).collect()
    }
}

impl Add {
    fn new(id: u64, left_num: u64, left_char: char, right_num: u64, right_char: char) -> Self {
        let (left, right) = Node::new(id, left_num, left_char, right_num, right_char);
        Add {
            id,
            left,
            right,
        }
    }
}