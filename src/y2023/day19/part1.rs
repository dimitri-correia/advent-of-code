use std::cmp::Ordering;
use std::collections::HashMap;

fn part_1(input: &str) -> String {
    let (workflows, parts): (HashMap<String, Workflow>, Vec<Part>) = parse_input(input);

    parts
        .iter()
        .filter_map(|p| follow_path(p, "in".to_string(), &workflows))
        .sum::<usize>()
        .to_string()
}

fn follow_path(
    p: &Part,
    condition_name: String,
    workflows: &HashMap<String, Workflow>,
) -> Option<usize> {
    let met_condition = workflows
        .get(&condition_name)
        .unwrap()
        .conditions
        .iter()
        .find(|condition| {
            let rule = condition.rule;
            if rule.cat == Cat::None {
                return true;
            }
            let v = match rule.cat {
                Cat::X => p.x,
                Cat::M => p.m,
                Cat::A => p.a,
                Cat::S => p.s,
                _ => panic!(),
            };
            match rule.comparator {
                Ordering::Less => v < rule.value,
                Ordering::Greater => v > rule.value,
                _ => panic!(),
            }
        })
        .unwrap();
    let res = met_condition.decision.clone();
    match res {
        Res::A => Some(p.x + p.m + p.a + p.s),
        Res::R => None,
        Res::Continue(condition_name) => follow_path(p, condition_name, workflows),
    }
}

#[derive(Debug, Clone)]
enum Res {
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
struct Workflow {
    conditions: Vec<Condition>,
}

#[derive(Debug)]
struct Condition {
    rule: Rule,
    decision: Res,
}

#[derive(Debug, Copy, Clone)]
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
            _ => panic!(),
        };
        let comparator = match rule_part_as_bytes[1] {
            b'<' => Ordering::Less,
            b'>' => Ordering::Greater,
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

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
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
        assert_eq!("401674", output);
    }

    #[test]
    fn example_test() {
        let input = include_str!("input1_ex.txt");
        let r = part_1(input);
        assert_eq!("19114", r);
    }
}
