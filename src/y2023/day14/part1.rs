use crate::y2023::day14::common::{get_val_col, parse_input};

fn part_1(input: &str) -> String {
    parse_input(input)
        .row_col
        .into_iter()
        .map(get_val_col)
        .sum::<usize>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn actual_challenge() {
        let input = include_str!("input1.txt");
        let output = part_1(input);
        dbg!(&output);
        assert_eq!("110407", output);
    }

    #[test]
    fn example_test() {
        let input = include_str!("input1_ex.txt"); // same file
        let r = part_1(input);
        assert_eq!("136", r);
    }
}
