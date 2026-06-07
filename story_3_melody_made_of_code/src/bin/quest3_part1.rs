use std::fs::read_to_string;

#[derive(Clone, Debug)]
struct Node {
    id: i32,
    plug: String,
    left_socket: String,
    right_socket: String,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    fn new(id: i32, plug: String, left_socket: String, right_socket: String) -> Self {
        Node {
            id,
            plug,
            left_socket,
            right_socket,
            left: None,
            right: None,
        }
    }

    fn insert(&mut self, node: Node) -> bool {
        if (self.left.is_none() && self.left_socket == node.plug) {
            self.left = Some(Box::new(node));
            return true;
        }

        if (self.right.is_none() && self.right_socket == node.plug) {
            self.right = Some(Box::new(node));
            return true;
        }

        if let Some(left) = &mut self.left {
            if (left.insert(node.clone())) {
                return true;
            }
        }

        if let Some(right) = &mut self.right {
            if (right.insert(node)) {
                return true;
            }
        }

        false
    }

    fn path(&self, list: &mut Vec<i32>) {
        if let Some(left) = &self.left {
            left.path(list);
        }
        list.push(self.id);
        if let Some(right) = &self.right {
            right.path(list);
        }
    }
}

fn main() {
    let file = read_to_string("input/quest3/part1.txt").unwrap();

    let mut lines = file.lines().map(|line| {
        let mut split = line.split(", ");
        let id = split.next().unwrap().replace("id=", "").parse::<i32>().unwrap();
        let plug = split.next().unwrap().replace("plug=", "");
        let left_socket = split.next().unwrap().replace("leftSocket=", "");
        let right_socket = split.next().unwrap().replace("rightSocket=", "");
        Node::new(id, plug, left_socket, right_socket)
    });

    let mut root = lines.next().unwrap();

    for node in lines {
        if !root.insert(node.clone()) {
            panic!("Failed to insert node: {:?}!", node);
        }
    }

    let mut list = Vec::new();
    root.path(&mut list);

    let result = list.into_iter()
        .enumerate()
        .map(|(x, y)| (x as i32 + 1) * y)
        .sum::<i32>();

    println!("Quest 3 part 1: {}", result);
}