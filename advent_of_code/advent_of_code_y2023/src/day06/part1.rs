use crate::day06::common::calculate_local_res;

pub fn part_1_example() -> String {
    let input = include_str!("input1_ex.txt");
    part_1(input)
}

pub fn part_1_actual_challenge() -> String {
    let input = include_str!("input1.txt");
    part_1(input)
}

fn part_1(input: &str) -> String {
    let (times, distances_to_beat): (Vec<usize>, Vec<usize>) = read_input(input);
    let res = compute_number_of_ways_to_win(times, distances_to_beat);
    res.to_string()
}

fn read_input(input: &str) -> (Vec<usize>, Vec<usize>) {
    let mut lines = input.lines().map(str::trim);

    let times = parse_line(lines.next().unwrap());
    let distances_to_beat = parse_line(lines.next().unwrap());

    (times, distances_to_beat)
}

fn parse_line(line: &str) -> Vec<usize> {
    line.split(':')
        .last()
        .unwrap()
        .split_whitespace()
        .map(|i| i.parse::<usize>().unwrap())
        .collect()
}

fn compute_number_of_ways_to_win(times: Vec<usize>, distances_to_beat: Vec<usize>) -> i32 {
    times
        .iter()
        .zip(distances_to_beat.iter())
        .map(|(&total_time, &distance_to_beat)| calculate_local_res(total_time, distance_to_beat))
        .product()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn actual_challenge() {
        let output = part_1_actual_challenge();        
        assert_eq!("4568778", output);
    }

    #[test]
    fn example_test() {
        let r = part_1_example();
        assert_eq!("288", r);
    }
}
