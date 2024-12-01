use super::common::parse_input;

pub fn part_1_example() -> String {
    let input = include_str!("input1_ex.txt");
    part_1(input)
}

pub fn part_1_actual_challenge() -> String {
    let input = include_str!("input1.txt");
    part_1(input)
}

fn part_1(input: &str) -> String {
    let (mut list_one, mut list_two) = parse_input(input);
    let mut result = 0;
    list_one.sort();
    list_two.sort();
    for i in 0..list_one.len() {
        result += (list_one[i] - list_two[i]).abs();
    }

    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn actual_challenge() {
        let output = part_1_actual_challenge();
        assert_eq!("2904518", output);
    }

    #[test]
    fn example_test() {
        let r = part_1_example();
        assert_eq!("11", r);
    }
}
