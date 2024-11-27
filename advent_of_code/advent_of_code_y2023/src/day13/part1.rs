use crate::day13::common::solve;
use itertools::Itertools;

pub fn part_1_example() -> String {
    let input = include_str!("input1_ex.txt");
    part_1(input)
}

pub fn part_1_actual_challenge() -> String {
    let input = include_str!("input1.txt");
    part_1(input)
}

fn part_1(input: &str) -> String {
    solve(input, find_fold)
}

fn find_fold(lines: Vec<&str>) -> Option<usize> {
    lines
        .iter()
        .enumerate()
        .tuple_windows()
        .filter(|((_, line_a), (_, line_b))| line_a == line_b)
        .find_map(|(_, (index_b, _))| {
            let (before, after) = lines.split_at(index_b);

            let all_lines_equal = before.iter().rev().zip(after).all(|(a, b)| a == b);

            all_lines_equal.then_some(index_b)
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn actual_challenge() {
        let output = part_1_actual_challenge();        
        assert_eq!("29213", output);
    }

    #[test]
    fn example_test() {
        let r = part_1_example();
        assert_eq!("405", r);
    }
}
