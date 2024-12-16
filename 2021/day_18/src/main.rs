use std::{
    cell::RefCell,
    fs::read_to_string,
    iter,
    option::Option,
    rc::{Rc, Weak},
};

use itertools::Itertools;
use regex::Regex;

#[derive(PartialEq, Eq, Clone, Copy)]
enum ChildDirection {
    Left,
    Right,
}

#[derive(Debug, Clone, Copy)]
enum Token {
    OpenBracket,
    Number(u32),
}

impl Token {
    fn from_str_iter(mut input: &str) -> impl Iterator<Item = Self> + '_ {
        let num_pat = Regex::new(r"^\d+").unwrap();
        let generate_token = move || loop {
            let next_char = input.chars().next()?;
            match next_char {
                c @ '[' => {
                    input = &input[c.len_utf8()..];
                    break Some(Token::OpenBracket);
                }

                d if d.is_ascii_digit() => {
                    let m = num_pat.find(input).unwrap().as_str();
                    let value = m.parse().unwrap();
                    input = &input[m.len()..];
                    break Some(Token::Number(value));
                }

                c => input = &input[c.len_utf8()..],
            }
        };

        iter::from_fn(generate_token)
    }
}

trait Node
where
    Self: Sized + Clone,
{
    type Inner;

    fn get_weak(&self) -> Weak<Self::Inner>;

    fn parent(&self) -> Option<(Self, ChildDirection)>;

    fn value(&self) -> NodeValue<Self>;

    fn new_leaf(value: u32, parent: Option<(Weak<Self::Inner>, ChildDirection)>) -> Self;

    fn new_branch(
        left: Self,
        right: Self,
        parent: Option<(Weak<Self::Inner>, ChildDirection)>,
    ) -> Self;

    fn modify(&self, f: impl FnOnce(&mut u32));

    fn set(&self, value: NodeValue<Self>);

    fn from_tokens(
        tokens: &mut impl Iterator<Item = Token>,
        parent: Option<(Weak<Self::Inner>, ChildDirection)>,
    ) -> Self {
        let next_token = tokens.next().unwrap();
        match next_token {
            Token::Number(x) => Self::new_leaf(x, parent),
            Token::OpenBracket => {
                let left = Self::from_tokens(tokens, None);
                let right = Self::from_tokens(tokens, None);
                Self::new_branch(left, right, parent)
            }
        }
    }

    fn from_str(input: &str) -> Self {
        let mut tokens = Token::from_str_iter(input);
        Self::from_tokens(&mut tokens, None)
    }

    fn depth(&self) -> usize {
        match &self.parent().map(|p| p.0) {
            None => 0,
            Some(n) => n.depth() + 1,
        }
    }

    fn get_leaf(&self) -> Option<u32> {
        match self.value() {
            NodeValue::Leaf(x) => Some(x),
            _ => None,
        }
    }

    fn get_branch(&self) -> Option<(Self, Self)> {
        match self.value() {
            NodeValue::Branch { left, right } => Some((left, right)),
            _ => None,
        }
    }

    fn modify_last(&self, f: impl FnOnce(&mut u32)) {
        match self.value() {
            NodeValue::Leaf(_) => self.modify(f),
            NodeValue::Branch { left: _, right } => right.modify_last(f),
        }
    }

    fn modify_previous_value(&self, f: impl FnOnce(&mut u32)) {
        let parent = match self.parent() {
            None => return,
            Some(p) => p,
        };

        let (left, _) = parent.0.get_branch().unwrap();
        let is_left_child = parent.1 == ChildDirection::Left;

        if is_left_child {
            parent.0.modify_previous_value(f);
        } else {
            left.modify_last(f);
        }
    }

    fn modify_first_value(&self, f: impl FnOnce(&mut u32)) {
        match self.value() {
            NodeValue::Leaf(_) => self.modify(f),
            NodeValue::Branch { left, right: _ } => left.modify_first_value(f),
        }
    }

    fn modify_next_value(&self, f: impl FnOnce(&mut u32)) {
        let parent = match self.parent() {
            None => return,
            Some(p) => p,
        };

        let (_, right) = parent.0.get_branch().unwrap();
        let is_right_child = parent.1 == ChildDirection::Right;

        if is_right_child {
            parent.0.modify_next_value(f);
        } else {
            right.modify_first_value(f);
        }
    }

    fn explode(&self) -> bool {
        let (left, right) = match self.value() {
            NodeValue::Branch { left, right } => (left, right),
            _ => return false,
        };

        if self.depth() == 4 {
            let (left, right) = (left.get_leaf().unwrap(), right.get_leaf().unwrap());

            self.modify_previous_value(|x| *x += left);
            self.modify_next_value(|x| *x += right);

            self.set(NodeValue::Leaf(0));
            return true;
        }

        if left.explode() {
            return true;
        }

        right.explode()
    }

    fn split(&self) -> bool {
        match self.value() {
            NodeValue::Leaf(x) => {
                if x < 10 {
                    return false;
                }

                let (left, right) = split_number(x);
                let left_leaf = Self::new_leaf(left, Some((self.get_weak(), ChildDirection::Left)));
                let right_leaf =
                    Self::new_leaf(right, Some((self.get_weak(), ChildDirection::Right)));
                self.set(NodeValue::Branch {
                    left: left_leaf,
                    right: right_leaf,
                });

                true
            }

            NodeValue::Branch { left, right } => {
                if left.split() {
                    return true;
                }

                right.split()
            }
        }
    }

    fn simplify(&mut self) {
        loop {
            if self.explode() {
                continue;
            }

            if self.split() {
                continue;
            }

            break;
        }
    }

    fn add(self, rhs: Self) -> Self {
        let mut output = Self::new_branch(self.clone(), rhs.clone(), None);
        output.simplify();
        output
    }

    fn magnitude(&self) -> u32 {
        match self.value() {
            NodeValue::Leaf(x) => x,
            NodeValue::Branch { left, right } => 3 * left.magnitude() + 2 * right.magnitude(),
        }
    }
}

enum NodeValue<T> {
    Leaf(u32),
    Branch { left: T, right: T },
}

struct BinaryTreeNode {
    parent: Option<(Weak<RefCell<BinaryTreeNode>>, ChildDirection)>,
    value: NodeValue<Rc<RefCell<BinaryTreeNode>>>,
}

impl Node for Rc<RefCell<BinaryTreeNode>> {
    type Inner = RefCell<BinaryTreeNode>;

    fn get_weak(&self) -> Weak<Self::Inner> {
        let rc = Rc::clone(self);
        Rc::downgrade(&rc)
    }
    fn parent(&self) -> Option<(Self, ChildDirection)> {
        self.borrow()
            .parent
            .as_ref()
            .and_then(|p| Some((p.0.upgrade()?, p.1)))
    }

    fn value(&self) -> NodeValue<Self> {
        match &self.borrow().value {
            NodeValue::Leaf(x) => NodeValue::Leaf(*x),
            NodeValue::Branch { left, right } => NodeValue::Branch {
                left: Rc::clone(left),
                right: Rc::clone(right),
            },
        }
    }

    fn new_leaf(
        value: u32,
        parent: Option<(Weak<RefCell<BinaryTreeNode>>, ChildDirection)>,
    ) -> Self {
        let leaf = BinaryTreeNode {
            value: NodeValue::Leaf(value),
            parent,
        };

        Rc::new(RefCell::new(leaf))
    }

    fn new_branch(
        left: Self,
        right: Self,
        parent: Option<(Weak<RefCell<BinaryTreeNode>>, ChildDirection)>,
    ) -> Self {
        Rc::new_cyclic(|this| {
            left.borrow_mut().parent = Some((this.clone(), ChildDirection::Left));
            right.borrow_mut().parent = Some((this.clone(), ChildDirection::Right));
            let value = NodeValue::Branch { left, right };
            let node = BinaryTreeNode { parent, value };
            RefCell::new(node)
        })
    }

    fn modify(&self, f: impl FnOnce(&mut u32)) {
        if let NodeValue::Leaf(ref mut x) = &mut self.borrow_mut().value {
            f(x);
        }
    }

    fn set(&self, value: NodeValue<Self>) {
        self.borrow_mut().value = value;
    }
}

fn split_number(x: u32) -> (u32, u32) {
    let half_rounded_down = x / 2;
    if x % 2 == 0 {
        (half_rounded_down, half_rounded_down)
    } else {
        (half_rounded_down, half_rounded_down + 1)
    }
}

fn solve(path: &str) -> (u32, u32) {
    let input = read_to_string(path).unwrap();
    let numbers: Vec<_> = input
        .lines()
        .map(<Rc<RefCell<BinaryTreeNode>> as Node>::from_str)
        .collect();

    let total_magnitude = numbers
        .into_iter()
        .reduce(|acc, x| acc.add(x))
        .unwrap()
        .magnitude();

    let max_magnitude = input
        .lines()
        .permutations(2)
        .map(|v| {
            [
                <Rc<RefCell<BinaryTreeNode>> as Node>::from_str(v[0]),
                <Rc<RefCell<BinaryTreeNode>> as Node>::from_str(v[1]),
            ]
        })
        .map(|v| v[0].clone().add(v[1].clone()).magnitude())
        .max()
        .unwrap();

    (total_magnitude, max_magnitude)
}

fn main() {
    let (output_1, output_2) = solve("input");
    println!("part 1: {output_1} part 2: {output_2}")
}

#[test]

fn practice() {
    let (output_1, output_2) = solve("practice");
    assert_eq!(output_1, 4140);
    assert_eq!(output_2, 3993)
}
