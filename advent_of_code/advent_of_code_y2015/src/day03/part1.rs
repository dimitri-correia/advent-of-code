use std::collections::HashSet;

pub fn part_1_example() -> String {
    let input = include_str!("input1_ex.txt");
    part_1(input)
}

pub fn part_1_actual_challenge() -> String {
    let input = include_str!("input1.txt");
    part_1(input)
}

fn part_1(input: &str) -> String {
    let mut visited = HashSet::new();
    visited.insert((0, 0));
    input.chars().fold((0, 0), |last, c| {
        let new = match c {
            '>' => (last.0 + 1, last.1),
            '<' => (last.0 - 1, last.1),
            '^' => (last.0, last.1 + 1),
            'v' => (last.0, last.1 - 1),
            _ => unreachable!(),
        };
        visited.insert(new);
        new
    });
    visited.len().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn actual_challenge() {
        let output = part_1_actual_challenge();        
        assert_eq!("2565", output);
    }

    #[test]
    fn example_test() {
        let r = part_1_example();
        assert_eq!("2", r);
    }
}
