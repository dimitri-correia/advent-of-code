fn part_1(input: &str) -> String {
    parse_input(input);
    "".to_string()
}

fn parse_input(input: &str) {
    input
        .lines()
        .map(|l| l.chars().map(|c| {}).collect())
        .collect()
}

#[derive(Debug)]
enum Type {
    Path,
    Forest,
    Slopes(SlopeType),
}

#[derive(Debug)]
enum SlopeType {
    Up,
    Down,
    Right,
    Left,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn actual_challenge() {
        let input = include_str!("input1.txt");
        let output = part_1(input);
        dbg!(&output);
        assert_eq!("", output);
    }

    #[test]
    fn example_test() {
        let input = include_str!("input1_ex.txt");
        let r = part_1(input);
        assert_eq!("94", r);
    }
}
