use std::cmp::max;
use std::collections::HashMap;

use crate::y2023::day02::common::{extract_game_info, Colors};

fn part_2(input: &str) -> String {
    input
        .lines()
        .map(|line| {
            extract_game_info(line)
                .subsets
                .iter()
                .fold(create_bag_min(), |mut acc, subset| {
                    subset.iter().for_each(|&(quantity, ref color)| {
                        let current = acc.entry(color.clone()).or_default();
                        *current = max(*current, quantity);
                    });
                    acc
                })
                .values()
                .product::<u32>()
        })
        .sum::<u32>()
        .to_string()
}

fn create_bag_min() -> HashMap<Colors, u32> {
    vec![(Colors::R, 0), (Colors::G, 0), (Colors::B, 0)]
        .into_iter()
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn actual_challenge() {
        let input = include_str!("input1.txt");
        let output = part_2(input);
        dbg!(&output);
        assert_eq!("75561", output);
    }

    #[test]
    fn example_test() {
        let input = include_str!("input1_ex.txt");
        let r = part_2(input);
        assert_eq!("2286", r);
    }
}
