use std::cmp::Ordering;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum Res {
    A,
    R,
    Continue(String),
}

impl Res {
    fn from_string(input: &str) -> Self {
        match input {
            "A" => Res::A,
            "R" => Res::R,
            _ => Res::Continue(input.to_string()),
        }
    }
}

#[derive(Debug)]
pub struct Workflow {
    pub conditions: Vec<Condition>,
}

#[derive(Debug)]
pub struct Condition {
    pub rule: Rule,
    pub decision: Res,
}

#[derive(Debug, Copy, Clone)]
pub struct Rule {
    pub cat: Cat,
    pub comparator: Ordering,
    pub value: usize,
}

impl Condition {
    fn from_string(input: &str) -> Self {
        if !input.contains(':') {
            let rule = Rule {
                cat: Cat::None,
                comparator: Ordering::Equal,
                value: 0,
            };
            return Condition {
                rule,
                decision: Res::from_string(input),
            };
        }
        let mut split = input.split(':');
        let rule_part = split.next().unwrap();
        let decision = Res::from_string(split.next().unwrap());
        let value = rule_part[2..].to_string().parse().unwrap();
        let rule_part_as_bytes = rule_part.as_bytes();
        let cat = match rule_part_as_bytes[0] {
            b'x' => Cat::X,
            b'm' => Cat::M,
            b'a' => Cat::A,
            b's' => Cat::S,
            _ => unreachable!(),
        };
        let comparator = match rule_part_as_bytes[1] {
            b'<' => Ordering::Less,
            b'>' => Ordering::Greater,
            _ => unreachable!(),
        };
        let rule = Rule {
            cat,
            comparator,
            value,
        };

        Condition { rule, decision }
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Cat {
    X,
    M,
    A,
    S,
    None,
}

#[derive(Debug)]
pub struct Part {
    pub x: usize,
    pub m: usize,
    pub a: usize,
    pub s: usize,
}

impl Part {
    fn from_string(input: &str) -> Self {
        let mut part = Part {
            x: 0,
            m: 0,
            a: 0,
            s: 0,
        };

        let pairs: Vec<&str> = input
            .trim_start_matches('{')
            .trim_end_matches('}')
            .split(',')
            .collect();

        for pair in pairs {
            let mut iter = pair.split('=');
            let key = iter.next().unwrap().trim();
            let value = iter.next().unwrap().trim().parse().unwrap();

            match key {
                "x" => part.x = value,
                "m" => part.m = value,
                "a" => part.a = value,
                "s" => part.s = value,
                _ => unreachable!(),
            }
        }

        part
    }
}

pub fn parse_input(input: &str) -> (HashMap<String, Workflow>, Vec<Part>) {
    let mut split = input.split("\n\n");
    let workflows: HashMap<String, Workflow> = split
        .next()
        .unwrap()
        .lines()
        .map(|l| {
            let mut split = l.split('{');
            let name = split.next().unwrap().trim().to_string();
            let pairs: Vec<&str> = split
                .next()
                .unwrap()
                .trim_start_matches('{')
                .trim_end_matches('}')
                .split(',')
                .collect();
            let conditions = pairs.iter().map(|p| Condition::from_string(p)).collect();
            let workflow = Workflow { conditions };
            (name, workflow)
        })
        .collect();

    let parts: Vec<Part> = split
        .next()
        .unwrap()
        .lines()
        .map(|l| Part::from_string(l))
        .collect();

    (workflows, parts)
}
