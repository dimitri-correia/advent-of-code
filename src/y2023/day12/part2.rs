use crate::y2023::day12::common::{get_nb_arrangements, parse_input, Line, SpringState};

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
        let input = include_str!("input1.txt");
        let output = part_2(input);
        dbg!(&output);
        assert_eq!("6720660274964", output);
    }

    #[test]
    fn example_test() {
        let input = include_str!("input1_ex.txt");
        let r = part_2(input);
        assert_eq!("525152", r);
    }
}
