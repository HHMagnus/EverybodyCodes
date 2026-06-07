
#[derive(Clone, Debug)]
struct Node {
    id: i32,
    plug: [String; 2],
    left_socket: [String; 2],
    right_socket: [String; 2],
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

fn plug_split(plug: String) -> [String; 2] {
    let mut plug_split = plug.split(" ");
    [plug_split.next().unwrap().to_string(), plug_split.next().unwrap().to_string()]
}

impl Node {
    fn new(id: i32, plug: String, left_socket: String, right_socket: String) -> Self {
        Node {
            id,
            plug: plug_split(plug),
            left_socket: plug_split(left_socket),
            right_socket: plug_split(right_socket),
            left: None,
            right: None,
        }
    }

    fn fits_left(&self, node: &Node, allow_weak_bonds: bool) -> bool {
        if !allow_weak_bonds {
            return self.left_socket == node.plug;
        }

        self.left_socket[0] == node.plug[0] || self.left_socket[1] == node.plug[1]
    }

    fn fits_right(&self, node: &Node, allow_weak_bonds: bool) -> bool {
        if !allow_weak_bonds {
            return self.right_socket == node.plug;
        }

        self.right_socket[0] == node.plug[0] || self.right_socket[1] == node.plug[1]
    }

    fn insert(&mut self, node: Node, allow_weak_bonds: bool) -> bool {
        if self.left.is_none() && self.fits_left(&node, allow_weak_bonds) {
            self.left = Some(Box::new(node));
            return true;
        }

        if let Some(left) = &mut self.left {
            if left.insert(node.clone(), allow_weak_bonds) {
                return true;
            }
        }

        if self.right.is_none() && self.fits_right(&node, allow_weak_bonds) {
            self.right = Some(Box::new(node));
            return true;
        }

        if let Some(right) = &mut self.right {
            if right.insert(node, allow_weak_bonds) {
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

pub fn quest3_solve(file: String, allow_week_bonds: bool) -> i32 {
    let mut lines = quest3_parse(file).into_iter();
    let mut root = lines.next().unwrap();

    for node in lines {
        if !root.insert(node.clone(), allow_week_bonds) {
            panic!("Failed to insert node: {:?}!", node);
        }
    }

    let mut list = Vec::new();
    root.path(&mut list);

    let result = list.into_iter()
        .enumerate()
        .map(|(x, y)| (x as i32 + 1) * y)
        .sum::<i32>();
    result
}

fn quest3_parse(file: String) -> Vec<Node> {
    file.lines().map(|line| {
        let mut split = line.split(", ");
        let id = split.next().unwrap().replace("id=", "").parse::<i32>().unwrap();
        let plug = split.next().unwrap().replace("plug=", "");
        let left_socket = split.next().unwrap().replace("leftSocket=", "");
        let right_socket = split.next().unwrap().replace("rightSocket=", "");
        Node::new(id, plug, left_socket, right_socket)
    }).collect()
}