pub fn part_1_example() -> String {
    let input = include_str!("input1_ex.txt");
    part_1(input)
}

pub fn part_1_actual_challenge() -> String {
    let input = include_str!("input1.txt");
    part_1(input)
}

fn part_1(input: &str) -> String {
    input
        .chars()
        .fold(0, |acc, c| match c {
            '(' => acc + 1,
            ')' => acc - 1,
            _ => unreachable!(),
        })
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn actual_challenge() {
        let output = part_1_actual_challenge();        
        assert_eq!("74", output);
    }

    #[test]
    fn example_test() {
        let r = part_1_example();
        assert_eq!("-3", r);
    }
}
