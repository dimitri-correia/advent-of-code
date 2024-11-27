use core::str;
use itertools::Itertools;

use super::common::{parse_input, Part, Receipe};

pub fn part_1_example() -> String {
    let input = include_str!("input1_ex.txt");
    part_1(input)
}

pub fn part_1_actual_challenge() -> String {
    let input = include_str!("input1.txt");
    part_1(input)
}

fn part_1(input: &str) -> String {
    let ingredients = parse_input(input);

    let ingredient_count = ingredients.len();
    vec![0..=100; ingredient_count]
        .into_iter()
        .multi_cartesian_product()
        .filter(|v| v.iter().sum::<i32>() == 100)
        .map(|qtt| {
            Receipe {
                ingredients: ingredients
                    .iter()
                    .cloned()
                    .zip(qtt.iter().cloned())
                    .collect(),
            }
            .compute_total_score(Part::Part1)
        })
        .max()
        .unwrap()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn actual_challenge() {
        let output = part_1_actual_challenge();
        assert_eq!("13882464", output);
    }

    #[test]
    fn example_test() {
        let r = part_1_example();
        assert_eq!("62842880", r);
    }
}
