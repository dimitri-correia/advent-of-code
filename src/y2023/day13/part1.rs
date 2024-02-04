use crate::y2023::day13::common::solve;
use itertools::Itertools;

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
        let input = include_str!("input1.txt");
        let output = part_1(input);
        dbg!(&output);
        assert_eq!("29213", output);
    }

    #[test]
    fn example_test() {
        let input = include_str!("input1_ex.txt"); // same file
        let r = part_1(input);
        assert_eq!("405", r);
    }
}
