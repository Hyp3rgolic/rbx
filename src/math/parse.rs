use crate::math::*;
use crate::math::prelude::*;

#[derive(Debug)]
pub enum Operation {
    Parenthesis(Vec<Operation>),
    Vector2(char,Vector2),
    Vector3(char,Vector3),
    Alphabetic(char),
    Add,
    Sub,
    V,
    U,
    Wedge,
    Mul,
    Div,
    Err,
}

fn match_chars(c: &str) -> Operation {
    if c != "e" {
        match c {
            "(" => Operation::Parenthesis(vec![]),
            ")" => Operation::Err,
            "v" => Operation::V,
            "u" => Operation::U,
            "+" => Operation::Add,
            "-" => Operation::Sub,
            "^" => Operation::Wedge,
            "/" => Operation::Div,
            "*" => Operation::Mul,
            _ => Operation::Err,
        }
    } else { return Operation::Vector2('a',Vector2 { x: 0.0, y: 0.0 }); }
}

pub fn parse_operation(s: String) -> Vec<Operation> {
    let mut queue: Vec<Operation> = vec![];
    let mut stack: Vec<Vec<Operation>> = vec![];

    for c in s.split(' ').filter(|s|!s.is_empty()) {
        match match_chars(c) {
            Operation::Err => {}
            Operation::Parenthesis(_) => stack.push(vec![]),
            op => {
                let top = stack.last_mut().unwrap_or(&mut queue);
                match op {
                    Operation::Parenthesis(sub) => top.push(Operation::Parenthesis(sub)),
                    op => top.push(op),
                }
            }
        }

        if c == ")" {
            let sub = stack.pop().unwrap();
            let top = stack.last_mut().unwrap_or(&mut queue);
            top.push(Operation::Parenthesis(sub));
        }
    }

    queue
}

pub fn compute(queue: Vec<Operation>) {
    let mut stack: Vec<Vec<Operation>> = vec![];
    for i in queue {
        match i {
            Operation::Vector2(..) => {}
            _ => {}
        }
    }
}

pub fn test_parse() {
    let input = String::from("v - u + ( v + u ) * 2");
    dbg!(parse_operation(input));
}