use common::read_path;
use std::collections::VecDeque;

#[derive(Debug)]
struct Node {
    value: i64,
    left: Option<Box<Node>>,
    middle: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    fn new(value: i64) -> Self {
        Node {
            value,
            left: None,
            middle: None,
            right: None,
        }
    }

    fn insert(&mut self, value: i64) {
        if let Some(ref mut left) = self.left {
            left.insert(value);
        } else {
            self.left = Some(Box::new(Node::new(self.value + value)));
        }
        if let Some(ref mut right) = self.right {
            right.insert(value);
        } else {
            self.right = Some(Box::new(Node::new(self.value * value)));
        }
    }

    fn insert_ternary(&mut self, value: i64) {
        if let Some(ref mut left) = self.left {
            left.insert_ternary(value);
        } else {
            self.left = Some(Box::new(Node::new(self.value + value)));
        }

        if let Some(ref mut middle) = self.middle {
            middle.insert_ternary(value);
        } else {
            let n_digits = value
                .to_string()
                .as_str()
                .bytes()
                .map(|b| (b - b'0') as u8)
                .collect::<Vec<u8>>();

            let base: i64 = 10;

            self.middle = Some(Box::new(Node::new(
                (self.value * base.pow(n_digits.len() as u32)) + value,
            )));
        }

        if let Some(ref mut right) = self.right {
            right.insert_ternary(value);
        } else {
            self.right = Some(Box::new(Node::new(self.value * value)));
        }
    }
}

fn dfs(root: Box<Node>, target: i64) -> bool {
    let mut queue = VecDeque::new();
    queue.push_back(&root);

    while let Some(node) = queue.pop_front() {
        if node.value == target
            && node.left.is_none()
            && node.right.is_none()
            && node.middle.is_none()
        {
            return true;
        }

        let items = node;
        if let Some(left) = &items.left {
            queue.push_front(left);
        }

        if let Some(middle) = &items.middle {
            queue.push_front(middle);
        }

        if let Some(right) = &items.right {
            queue.push_front(right);
        }
    }
    false
}

fn build_tree(equation: &Vec<i64>) -> Node {
    let mut tree = Node::new(equation[0]);
    for num in equation[1..].iter() {
        tree.insert(*num);
    }
    tree
}

fn build_ternary_tree(equation: &Vec<i64>) -> Node {
    let mut tree = Node::new(equation[0]);
    for num in equation[1..].iter() {
        tree.insert_ternary(*num);
    }
    tree
}

fn is_valid(target: i64, equation: &Vec<i64>) -> bool {
    let tree = build_tree(equation);
    dfs(Box::new(tree), target)
}

fn is_valid_ternary(target: i64, equation: &Vec<i64>) -> bool {
    let tree = build_ternary_tree(equation);
    dfs(Box::new(tree), target)
}

fn get_targets_and_equations(input: String) -> (Vec<i64>, Vec<Vec<i64>>) {
    let mut targets = vec![];
    let mut equations = vec![];

    for line in input.lines() {
        let data: Vec<&str> = line.split(": ").collect();
        targets.push(
            data[0]
                .parse::<i64>()
                .expect("Failed to parse target to i64."),
        );
        let mut equation = vec![];
        for num in data[1].split(' ') {
            equation.push(num.parse::<i64>().expect("Failed to parse number to i64."));
        }

        equations.push(equation);
    }

    (targets, equations)
}

mod solutions {
    use std::fs;

    use crate::{get_targets_and_equations, is_valid, is_valid_ternary};

    pub fn part_1_solution(path: &str) -> i64 {
        let input = fs::read_to_string(path).expect("Failed to read input.");

        let (targets, equations) = get_targets_and_equations(input);

        let mut res = 0;

        for (target, equation) in targets.iter().zip(equations) {
            if is_valid(*target, &equation) {
                res += target;
            }
        }
        res
    }

    pub fn part_2_solution(path: &str) -> i64 {
        let input = fs::read_to_string(path).expect("Failed to read input.");

        let (targets, equations) = get_targets_and_equations(input);

        let mut res = 0;

        for (target, equation) in targets.iter().zip(equations) {
            if is_valid(*target, &equation) {
                res += target;
            } else if is_valid_ternary(*target, &equation) {
                res += target;
            }
        }
        res
    }
}

fn main() {
    let path = read_path();

    let solution = solutions::part_1_solution(&path);

    println!("{solution}");
    let solution = solutions::part_2_solution(&path);

    println!("{solution}");
}
