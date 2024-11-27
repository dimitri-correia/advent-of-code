use core::str;

use super::common::parse_input;

pub fn part_1_example() -> String {
    let input = include_str!("input1_ex.txt");
    part_1(input, 1000)
}

pub fn part_1_actual_challenge() -> String {
    let input = include_str!("input1.txt");
    part_1(input, 2503)
}

fn part_1(input: &str, time_in_s: u16) -> String {
    let reindeers = parse_input(input);
    reindeers
        .iter()
        .map(|r| r.distance_after(time_in_s))
        .max()
        .unwrap()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn actual_challenge() {
        let output = part_1_actual_challenge();        assert_eq!("2696", output);
    }

    #[test]
    fn example_test() {
        let r = part_1_example();
        assert_eq!("1120", r);
    }
}
