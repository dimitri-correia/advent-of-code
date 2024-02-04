use crate::y2023::day14::common::{get_val_col, parse_input, Shape};

fn part_2(input: &str) -> String {
    const NUMBER_CYCLE: usize = 1_000_000_000;
    let grid = parse_input(input);
    let final_grid = (0..NUMBER_CYCLE)
        .into_iter()
        .fold(grid, |tmp_grid, _| do_one_cycle(tmp_grid));

    final_grid
        .into_iter()
        .map(get_val_col)
        .sum::<usize>()
        .to_string()
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum TiltDir {
    North,
    West,
    South,
    East,
}

fn do_one_cycle(grid: Vec<Vec<Shape>>) -> Vec<Vec<Shape>> {
    grid
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn actual_challenge() {
        let input = include_str!("input1.txt");
        let output = part_2(input);
        dbg!(&output);
        assert_eq!("val", output);
    }

    #[test]
    fn example_test() {
        let input = include_str!("input1_ex.txt"); // same file
        let r = part_2(input);
        assert_eq!("64", r);
    }
}
