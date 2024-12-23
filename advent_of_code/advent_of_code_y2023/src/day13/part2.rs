use crate::day13::common::solve;
use itertools::Itertools;

pub fn part_2_example() -> String {
    let input = include_str!("input1_ex.txt");
    part_2(input)
}

pub fn part_2_actual_challenge() -> String {
    let input = include_str!("input1.txt");
    part_2(input)
}

fn part_2(input: &str) -> String {
    solve(input, find_fold)
}

fn find_fold(lines: Vec<&str>) -> Option<usize> {
    lines
        .iter()
        .enumerate()
        .tuple_windows()
        .filter(|((_, line_a), (_, line_b))| {
            line_a
                .chars()
                .zip(line_b.chars())
                .filter(|(a, b)| a != b)
                .count()
                <= 1
        })
        .find_map(|(_, (index_b, _))| {
            let (before, after) = lines.split_at(index_b);

            let all_lines_equal = 1
                == before
                    .iter()
                    .rev()
                    .join("")
                    .chars()
                    .zip(after.join("").chars())
                    .filter(|(a, b)| a != b)
                    .count();

            all_lines_equal.then_some(index_b)
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn actual_challenge() {
        let output = part_2_actual_challenge();        assert_eq!("37453", output);
    }

    #[test]
    fn example_test() {
        let r = part_2_example();
        assert_eq!("400", r);
    }
}
