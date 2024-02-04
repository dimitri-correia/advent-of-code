use crate::y2023::day14::common::{parse_input, Shape};

fn part_1(input: &str) -> String {
    parse_input(input)
        .into_iter()
        .map(get_val_col)
        .sum::<usize>()
        .to_string()
}

fn get_val_col(col: Vec<Shape>) -> usize {
    let len = col.len();

    let mut r = 0;
    let mut curr_rounded_tot = 0;
    let mut last_non_cube_rock = len;

    for (idx, shape) in col.iter().enumerate() {
        if shape == &Shape::CubeRock {
            r += (last_non_cube_rock - curr_rounded_tot + 1..=last_non_cube_rock).sum::<usize>();
            curr_rounded_tot = 0;
            last_non_cube_rock = len - idx - 1;
        } else if shape == &Shape::RoundedRock {
            curr_rounded_tot += 1;
        }
    }

    r + (last_non_cube_rock - curr_rounded_tot + 1..=last_non_cube_rock).sum::<usize>()
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
