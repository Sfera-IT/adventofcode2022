use crate::utils;
use std::collections::{HashMap, VecDeque};
use std::iter::Iterator;

enum OpType {
    Add,
    Sub,
    Mul,
    Div,
}

enum Yell {
    Num(i64),
    Op(OpType, String, String),
}

struct Monkey {
    yell: Yell,
    name: String,
}

impl Monkey {
    pub fn parse(line: String) -> Option<Self> {
        let mut chars = VecDeque::from_iter(line.chars());
        let name = Self::parse_name(&mut chars);
        Self::discard_char(&mut chars, ':');
        Self::discard_char(&mut chars, ' ');

        let yell = match chars.front() {
            None => panic!("Unexpected end of string"),
            Some(c) if c.is_ascii_digit() => Yell::Num(Self::parse_number(&mut chars)),
            Some(_) => {
                let operand1 = Self::parse_name(&mut chars);
                Self::discard_char(&mut chars, ' ');
                let op_type = match chars.pop_front() {
                    None => panic!("Operand expected"),
                    Some('+') => OpType::Add,
                    Some('-') => OpType::Sub,
                    Some('*') => OpType::Mul,
                    Some('/') => OpType::Div,
                    Some(c) => panic!("Unexpected operand '{}'", c),
                };
                Self::discard_char(&mut chars, ' ');
                let operand2 = Self::parse_name(&mut chars);
                Yell::Op(op_type, operand1, operand2)
            }
        };

        Some(Self { name, yell })
    }

    fn parse_name(chars: &mut VecDeque<char>) -> String {
        let mut s = String::with_capacity(4);
        s.push(chars.pop_front().expect("Character expected"));
        s.push(chars.pop_front().expect("Character expected"));
        s.push(chars.pop_front().expect("Character expected"));
        s.push(chars.pop_front().expect("Character expected"));
        s
    }

    fn discard_char(chars: &mut VecDeque<char>, expected: char) {
        match chars.pop_front() {
            None => panic!("Character '{}' expected, found nothing", expected),
            Some(c) if c == expected => (),
            Some(c) => panic!("Character '{}' expected, found '{}'", expected, c),
        }
    }

    fn parse_number(chars: &mut VecDeque<char>) -> i64 {
        let s = chars.iter().collect::<String>();
        s.parse::<i64>().expect("Number expected")
    }
}

fn resolve_monkey(monkeys: &HashMap<String, Monkey>, name: &str) -> Option<i64> {
    let Some(m) = monkeys.get(name) else { return None };

    match &m.yell {
        Yell::Num(n) => Some(*n),
        Yell::Op(OpType::Add, op1, op2) => {
            Some(resolve_monkey(monkeys, op1)? + resolve_monkey(monkeys, op2)?)
        }
        Yell::Op(OpType::Sub, op1, op2) => {
            Some(resolve_monkey(monkeys, op1)? - resolve_monkey(monkeys, op2)?)
        }
        Yell::Op(OpType::Mul, op1, op2) => {
            Some(resolve_monkey(monkeys, op1)? * resolve_monkey(monkeys, op2)?)
        }
        Yell::Op(OpType::Div, op1, op2) => {
            Some(resolve_monkey(monkeys, op1)? / resolve_monkey(monkeys, op2)?)
        }
    }
}

fn resolve_monkey_inverse(monkeys: &HashMap<String, Monkey>, name: &str, target: i64) -> i64 {
    if name == "humn" {
        return target;
    }

    let node = monkeys.get(name).expect("node not found");
    let Yell::Op(op_type, branch1, branch2) = &node.yell else { panic!("name is not an operation") };

    let res1 = resolve_monkey(monkeys, branch1);
    let res2 = resolve_monkey(monkeys, branch2);

    let (human_branch, op_value, human_left_side) = match (res1, res2) {
        (None, None) => panic!("Human on both sides"),
        (Some(_), Some(_)) => panic!("Human on neither side"),
        (None, Some(v)) => (branch1, v, true),
        (Some(v), None) => (branch2, v, false),
    };

    let human_branch_target = match (op_type, human_left_side) {
        (OpType::Add, _) => target - op_value,
        (OpType::Sub, true) => target + op_value,
        (OpType::Sub, false) => op_value - target,
        (OpType::Mul, _) => target / op_value,
        (OpType::Div, true) => target * op_value,
        (OpType::Div, false) => op_value / target,
    };

    resolve_monkey_inverse(monkeys, human_branch, human_branch_target)
}

pub fn test1() {
    let mut monkeys = HashMap::new();

    for m in utils::parse_lines("./data/day21.txt", Monkey::parse).drain(..) {
        monkeys.insert(m.name.clone(), m);
    }

    let root = resolve_monkey(&monkeys, "root");
    println!("Root: {}", root.unwrap());
}

pub fn test2() {
    let mut monkeys = HashMap::new();

    for m in utils::parse_lines("./data/day21.txt", Monkey::parse).drain(..) {
        if m.name != "humn" {
            monkeys.insert(m.name.clone(), m);
        }
    }

    let root = monkeys.get_mut("root").expect("root not found");
    let Yell::Op(_, branch1, branch2) = &root.yell else { panic!("root is not an operation") };
    root.yell = Yell::Op(OpType::Sub, branch1.clone(), branch2.clone());

    let human_value = resolve_monkey_inverse(&monkeys, "root", 0);
    println!("HUMN: {}", human_value);
}
