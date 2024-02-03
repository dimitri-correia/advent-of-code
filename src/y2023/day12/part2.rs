use crate::y2023::day12::common::{
    count_unknown, get_nb_arrangements, parse_str_to_spring_state, Line, SpringState,
};

fn part_2(input: &str) -> String {
    parse_input(input)
        .iter()
        .map(|line| get_nb_arrangements(line))
        .sum::<usize>()
        .to_string()
}

fn parse_input(input: &str) -> Vec<Line> {
    input
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line.split_whitespace().collect();
            let line_state: Vec<SpringState> = parts[0]
                .chars()
                .map(|c| parse_str_to_spring_state(c))
                .chain(std::iter::once(SpringState::Unknown))
                .cycle()
                .take(parts[0].len() * 5 + 4)
                .collect();

            let order: Vec<usize> = parts[1]
                .split(',')
                .map(|s| s.parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
                .repeat(5);

            let count_unknown = count_unknown(&line_state);

            Line {
                line_state,
                order,
                count_unknown,
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn actual_challenge() {
        panic!();
        let input = include_str!("input1.txt");
        let output = part_2(input);
        dbg!(&output);
        assert_eq!("", output);
    }

    #[test]
    fn example_test() {
        panic!();
        let input = include_str!("input1_ex.txt"); // same file
        let r = part_2(input);
        assert_eq!("525152", r);
    }
}
