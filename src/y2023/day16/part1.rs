use crate::y2023::day16::common::{get_number_energized_tiles, parse_input, Coord, Dir};

fn part_1(input: &str) -> String {
    let grid = parse_input(input);
    get_number_energized_tiles(&grid, Dir::Right, Coord(0, 0)).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn actual_challenge() {
        let input = include_str!("input1.txt");
        let output = part_1(input);
        dbg!(&output);
        assert_eq!("7798", output);
    }

    #[test]
    fn example_test() {
        let input = include_str!("input1_ex.txt");
        let r = part_1(input);
        assert_eq!("46", r);
    }
}
