use crate::day06::common::calculate_local_res;

pub fn part_2_example() -> String {
    let input = include_str!("input1_ex.txt");
    part_2(input)
}

pub fn part_2_actual_challenge() -> String {
    let input = include_str!("input1.txt");
    part_2(input)
}

fn part_2(input: &str) -> String {
    let (time, distance_to_beat) = read_input(input);
    calculate_local_res(time, distance_to_beat).to_string()
}

fn read_input(input: &str) -> (usize, usize) {
    let mut lines = input.lines().map(str::trim);

    let time = parse_line(lines.next().unwrap());
    let distance_to_beat = parse_line(lines.next().unwrap());

    (time, distance_to_beat)
}

fn parse_line(line: &str) -> usize {
    line.chars()
        .filter(|c| c.is_numeric())
        .collect::<String>()
        .parse::<usize>()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn actual_challenge() {
        let output = part_2_actual_challenge();        assert_eq!("28973936", output);
    }

    #[test]
    fn example_test() {
        let r = part_2_example();
        assert_eq!("71503", r);
    }
}
