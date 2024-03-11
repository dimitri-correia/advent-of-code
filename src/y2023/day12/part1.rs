use crate::y2023::day12::common::{parse_str_to_spring_state, Line, SpringState};

fn part_1(input: &str) -> String {
    parse_input(input)
        .iter()
        .map(|line| get_nb_arrangements(line))
        .sum::<usize>()
        .to_string()
}

fn get_nb_arrangements(p0: &Line) -> B {
    todo!()
}

fn parse_input(input: &str) -> Vec<Line> {
    input
        .lines()
        .map(|line| {
            let (line_state, order) = line.split_once(' ').unwrap();
            let line_state: Vec<SpringState> = line_state
                .chars()
                .map(|c| parse_str_to_spring_state(c))
                .collect();
            let order: Vec<usize> = order
                .split(',')
                .map(|s| s.parse::<usize>().unwrap())
                .collect();
            Line { line_state, order }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn actual_challenge() {
        let input = include_str!("input1.txt");
        let output = part_1(input);
        dbg!(&output);
        assert_eq!("7460", output);
    }

    #[test]
    fn example_test() {
        let input = include_str!("input1_ex.txt");
        let r = part_1(input);
        assert_eq!("21", r);
    }
}
