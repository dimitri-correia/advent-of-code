use std::collections::HashMap;

use crate::day02::common::{extract_game_info, Colors};

pub fn part_1_example() -> String {
    let input = include_str!("input1_ex.txt");
    part_1(input)
}

pub fn part_1_actual_challenge() -> String {
    let input = include_str!("input1.txt");
    part_1(input)
}

fn part_1(input: &str) -> String {
    let bag_possibilities = get_bag_possibilities();
    input
        .lines()
        .map(|line| {
            let game_info = extract_game_info(line);
            let condition = !game_info.subsets.iter().any(|s| {
                s.iter()
                    .any(|(qtt, color)| bag_possibilities.get(color) < Some(qtt))
            });
            return if condition { game_info.game_id } else { 0 };
        })
        .sum::<u32>()
        .to_string()
}

fn get_bag_possibilities() -> HashMap<Colors, u32> {
    vec![(Colors::R, 12), (Colors::G, 13), (Colors::B, 14)]
        .into_iter()
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn actual_challenge() {
        let output = part_1_actual_challenge();        
        assert_eq!("2795", output);
    }

    #[test]
    fn example_test() {
        let input = include_str!("input1_ex.txt");

        let r = part_1(input);

        assert_eq!("8", r);
    }
}
