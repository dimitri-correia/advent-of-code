use crate::day12::common::{get_nb_arrangements, parse_input, Line, SpringState};

pub fn part_2_example() -> String {
    let input = include_str!("input1_ex.txt");
    part_2(input)
}

pub fn part_2_actual_challenge() -> String {
    let input = include_str!("input1.txt");
    part_2(input)
}

fn part_2(input: &str) -> String {
    parse_input_and_expand(input, 5)
        .iter()
        .map(|line| get_nb_arrangements(line))
        .sum::<usize>()
        .to_string()
}

fn parse_input_and_expand(input: &str, nb_expand: usize) -> Vec<Line> {
    let lines = parse_input(input);
    lines
        .iter()
        .map(|line| {
            let line_state = line
                .line_state
                .clone()
                .into_iter()
                .chain(std::iter::once(SpringState::Unknown))
                .cycle()
                .take(line.line_state.len() * 5 + 4)
                .collect();
            let groups = line.groups.repeat(nb_expand);
            Line { line_state, groups }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn actual_challenge() {
        let output = part_2_actual_challenge();        assert_eq!("6720660274964", output);
    }

    #[test]
    fn example_test() {
        let r = part_2_example();
        assert_eq!("525152", r);
    }
}
