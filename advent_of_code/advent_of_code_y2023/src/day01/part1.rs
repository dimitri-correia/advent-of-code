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
        let output = part_1_actual_challenge();        
        assert_eq!("53386", output);
    }

    #[test]
    fn example_test() {
        let r = part_1_example();
        assert_eq!("142", r);
    }
}
