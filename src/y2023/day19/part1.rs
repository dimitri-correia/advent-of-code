use std::cmp::Ordering;
use std::collections::HashMap;

fn part_1(input: &str) -> String {
    let (workflows, parts) = parse_input(input);

    dbg!(workflows, parts);
    "".to_string()
}

#[derive(Debug)]
struct Workflow {
    conditions: Vec<Condition>,
}

#[derive(Debug)]
struct Condition {
    rule: Rule,
    decision: String,
}

#[derive(Debug)]
struct Rule {
    cat: Cat,
    comparator: Ordering,
    value: usize,
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
                decision: input.to_string(),
            };
        }
        let mut split = input.split(':');
        let rule_part = split.next().unwrap();
        let decision = split.next().unwrap().to_string();
        let value = rule_part[2..].to_string().parse().unwrap();
        let rule_part_as_bytes = rule_part.as_bytes();
        let cat = match rule_part_as_bytes[0] {
            b'x' => Cat::X,
            b'm' => Cat::M,
            b'a' => Cat::A,
            b's' => Cat::S,
            _ => panic!(),
        };
        let comparator = match rule_part_as_bytes[1] {
            b'<' => Ordering::Greater,
            b'>' => Ordering::Less,
            _ => panic!(),
        };
        let rule = Rule {
            cat,
            comparator,
            value,
        };

        Condition { rule, decision }
    }
}

#[derive(Debug)]
enum Cat {
    X,
    M,
    A,
    S,
    None,
}

#[derive(Debug)]
struct Part {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
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
                _ => panic!(),
            }
        }

        part
    }
}

fn parse_input(input: &str) -> (HashMap<String, Workflow>, Vec<Part>) {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn actual_challenge() {
        let input = include_str!("input1.txt");
        let output = part_1(input);
        dbg!(&output);
        assert_eq!("", output);
    }

    #[test]
    fn example_test() {
        let input = include_str!("input1_ex.txt");
        let r = part_1(input);
        assert_eq!("19114", r);
    }
}
