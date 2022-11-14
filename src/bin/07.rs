use anyhow::Result;
use std::{cell::RefCell, collections::HashMap, str::FromStr};

#[derive(Debug)]
enum Term {
    Var(String),
    Num(u16),
}

impl FromStr for Term {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self> {
        if let Ok(num) = s.parse::<u16>() {
            Ok(Term::Num(num))
        } else {
            Ok(Term::Var(s.to_string()))
        }
    }
}

#[derive(Debug)]
enum Instruction {
    Num(u16),
    Var(String),
    Or(Term, Term),
    And(Term, Term),
    Rshift(String, u16),
    Lshift(String, u16),
    Not(String),
}

impl FromStr for Instruction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        match s.split(' ').collect::<Vec<&str>>()[..] {
            [x, "AND", y] => Ok(Instruction::And(x.parse()?, y.parse()?)),
            [x, "OR", y] => Ok(Instruction::Or(x.parse()?, y.parse()?)),
            ["NOT", x] => Ok(Instruction::Not(x.to_string())),
            [x, "LSHIFT", y] => Ok(Instruction::Lshift(x.to_string(), y.parse::<u16>()?)),
            [x, "RSHIFT", y] => Ok(Instruction::Rshift(x.to_string(), y.parse::<u16>()?)),
            [x] => match x.parse::<u16>() {
                Ok(v) => Ok(Instruction::Num(v)),
                Err(_) => Ok(Instruction::Var(x.to_string())),
            },
            _ => panic!("invalid input"),
        }
    }
}

#[derive(Debug)]
struct Solution {
    instructions: HashMap<String, Instruction>,
    wires: RefCell<HashMap<String, u16>>,
}

impl Solution {
    fn new(input: &str) -> Self {
        let mut instructions = HashMap::new();

        input.lines().for_each(|line| {
            let (a, b) = line.rsplit_once(" -> ").unwrap();
            instructions.insert(b.to_string(), a.parse().unwrap());
        });
        Self {
            instructions,
            wires: RefCell::new(HashMap::new()),
        }
    }

    fn get_value(&self, var: &Term) -> u16 {
        let var = match var {
            Term::Num(n) => return *n,
            Term::Var(var) => var,
        };

        if let Some(val) = self.wires.borrow().get(var) {
            return *val;
        };

        let val = match self.instructions.get(var).unwrap() {
            Instruction::Num(val) => *val,
            Instruction::Var(name) => self.get_value(&name.parse().unwrap()),
            Instruction::Or(left, right) => {
                let left = self.get_value(left);
                let right = self.get_value(right);
                left | right
            }
            Instruction::And(left, right) => {
                let left = self.get_value(left);
                let right = self.get_value(right);
                left & right
            }
            Instruction::Lshift(name, amt) => {
                let name = self.get_value(&name.parse().unwrap());
                name << amt
            }
            Instruction::Rshift(name, amt) => {
                let name = self.get_value(&name.parse().unwrap());
                name >> amt
            }
            Instruction::Not(name) => {
                let name = self.get_value(&name.parse().unwrap());
                !name
            }
        };
        self.wires.borrow_mut().insert(var.to_string(), val);
        val
    }
}

pub fn part_one(input: &str) -> Option<u16> {
    let parsed = Solution::new(input);
    Some(parsed.get_value(&"a".parse().unwrap()))
}

pub fn part_two(input: &str) -> Option<u16> {
    let parsed = Solution::new(input);
    parsed.wires.borrow_mut().insert("b".to_string(), 46065);
    Some(parsed.get_value(&"a".parse().unwrap()))
}

fn main() {
    let input = &aoc::read_file("inputs", 7);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}
