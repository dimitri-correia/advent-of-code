use crate::day16::common::{get_number_energized_tiles, parse_input, Coord, Dir};

pub fn part_1_example() -> String {
    let input = include_str!("input1_ex.txt");
    part_1(input)
}

pub fn part_1_actual_challenge() -> String {
    let input = include_str!("input1.txt");
    part_1(input)
}

fn part_1(input: &str) -> String {
    let grid = parse_input(input);
    get_number_energized_tiles(&grid, Dir::Right, Coord(0, 0)).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn actual_challenge() {
        let output = part_1_actual_challenge();        
        assert_eq!("7798", output);
    }

    #[test]
    fn example_test() {
        let r = part_1_example();
        assert_eq!("46", r);
    }
}
