fn part_1(input: &str) -> String {
    input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            let first_digit = get_first_digit(line.chars());
            let last_digit = get_first_digit(line.chars().rev());

            10 * first_digit + last_digit
        })
        .sum::<u32>()
        .to_string()
}

fn get_first_digit<T>(mut chars: T) -> u32
where
    T: Iterator<Item = char>,
{
    chars
        .find(|c| c.is_ascii_digit())
        .unwrap()
        .to_digit(10)
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn actual_challenge() {
        let input = include_str!("input1.txt");
        let output = part_1(input);
        dbg!(&output);
        assert_eq!("53386", output);
    }

    #[test]
    fn example_test() {
        let input = include_str!("input1_ex.txt");
        let r = part_1(input);
        assert_eq!("142", r);
    }
}
