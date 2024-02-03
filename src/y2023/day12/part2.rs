use crate::y2023::day12::common::{
    get_nb_arrangements, parse_str_to_spring_state, Line, SpringState,
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
                .flat_map(|num| std::iter::repeat(num).take(5))
                .collect();
            let order: Vec<usize> = parts[1]
                .split(',')
                .map(|s| s.parse::<usize>().unwrap())
                .flat_map(|num| std::iter::repeat(num).take(5))
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
        let output = part_2(input);
        dbg!(&output);
        assert_eq!("", output);
    }

    #[test]
    fn example_test() {
        let input = include_str!("input1_ex.txt"); // same file
        let r = part_2(input);
        assert_eq!("525152", r);
    }
}
