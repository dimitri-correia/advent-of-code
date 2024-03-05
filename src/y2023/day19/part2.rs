use crate::y2023::day19::common::{parse_input, Cat, Part, Res, Rule, Workflow};
use std::cmp::{max, min, Ordering};
use std::collections::HashMap;
use std::ops::RangeInclusive;

fn part_2(input: &str) -> String {
    let (workflows, _): (HashMap<String, Workflow>, Vec<Part>) = parse_input(input);

    follow_path(
        PartRange {
            x: 1..=4000,
            m: 1..=4000,
            a: 1..=4000,
            s: 1..=4000,
        },
        "in".to_string(),
        &workflows,
    )
    .to_string()
}

#[derive(Debug, Clone)]
pub struct PartRange {
    pub x: RangeInclusive<usize>,
    pub m: RangeInclusive<usize>,
    pub a: RangeInclusive<usize>,
    pub s: RangeInclusive<usize>,
}

impl PartRange {
    pub fn combinations(&self) -> usize {
        let x_count = self.x.end() - self.x.start() + 1;
        let m_count = self.m.end() - self.m.start() + 1;
        let a_count = self.a.end() - self.a.start() + 1;
        let s_count = self.s.end() - self.s.start() + 1;

        dbg!(x_count * m_count * a_count * s_count)
    }
}

fn follow_path(
    start_range: PartRange,
    start: String,
    workflows: &HashMap<String, Workflow>,
) -> usize {
    dbg!(&start, &start_range);
    workflows
        .get(&start)
        .unwrap()
        .conditions
        .iter()
        .fold((start_range, 0), |(range, sum), condition| {
            let (range_to_pass, range_to_keep) = get_new_range(&range, condition.rule);
            let new_sum = match condition.decision.clone() {
                Res::A => range.combinations(),
                Res::R => 0,
                Res::Continue(condition_name) => {
                    follow_path(range_to_pass.clone(), condition_name, workflows)
                }
            };
            (range_to_keep, sum + new_sum)
        })
        .1
}

fn get_new_range(old_range: &PartRange, rule: Rule) -> (PartRange, PartRange) {
    if Cat::None == rule.cat {
        return (old_range.clone(), old_range.clone());
    }
    let mut range_to_pass = old_range.clone();
    let mut range_to_keep = old_range.clone();
    let pass_to_update = get_part_of_range_to_update(rule, &mut range_to_pass);
    let keep_to_update = get_part_of_range_to_update(rule, &mut range_to_keep);
    *pass_to_update = match rule.comparator {
        Ordering::Less => *pass_to_update.start()..=min(*pass_to_update.end(), rule.value),
        Ordering::Greater => max(*pass_to_update.start(), rule.value)..=*pass_to_update.end(),
        _ => unreachable!(),
    };
    *keep_to_update = match rule.comparator {
        Ordering::Greater => *keep_to_update.start()..=min(*keep_to_update.end(), rule.value),
        Ordering::Less => max(*keep_to_update.start(), rule.value)..=*keep_to_update.end(),
        _ => unreachable!(),
    };
    (range_to_pass, range_to_keep)
}

fn get_part_of_range_to_update(
    rule: Rule,
    range_to_pass: &mut PartRange,
) -> &mut RangeInclusive<usize> {
    let range_to_update: &mut RangeInclusive<usize> = match rule.cat {
        Cat::X => &mut range_to_pass.x,
        Cat::M => &mut range_to_pass.m,
        Cat::A => &mut range_to_pass.a,
        Cat::S => &mut range_to_pass.s,
        _ => unreachable!(),
    };
    range_to_update
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn actual_challenge() {
        let input = include_str!("input1.txt");
        let output = part_2(input);
        dbg!(&output);
        assert_eq!("", output);
    }
    #[test]
    fn example_test() {
        let input = include_str!("input1_ex.txt");
        let r = part_2(input);
        assert_eq!("167409079868000", r);
    }
}
