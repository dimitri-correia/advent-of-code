use crate::day19::common::{parse_input, Cat, Part, Res, Workflow};
use std::cmp::Ordering;
use std::collections::HashMap;

pub fn part_1_example() -> String {
    let input = include_str!("input1_ex.txt");
    part_1(input)
}

pub fn part_1_actual_challenge() -> String {
    let input = include_str!("input1.txt");
    part_1(input)
}

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
                _ => unreachable!(),
            };
            match rule.comparator {
                Ordering::Less => v < rule.value,
                Ordering::Greater => v > rule.value,
                _ => unreachable!(),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn actual_challenge() {
        let output = part_1_actual_challenge();        
        assert_eq!("401674", output);
    }

    #[test]
    fn example_test() {
        let r = part_1_example();
        assert_eq!("19114", r);
    }
}
