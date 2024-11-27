pub fn part_2_example() -> String {
    let input = include_str!("input1_ex.txt");
    part_2(input)
}

pub fn part_2_actual_challenge() -> String {
    let input = include_str!("input1.txt");
    part_2(input)
}

fn part_2(input: &str) -> String {
    input
        .lines()
        .map(|l| get_encoded_str(l) - l.len())
        .sum::<usize>()
        .to_string()
}

fn get_encoded_str(l: &str) -> usize {
    let mut count = 2;
    let mut iter = l.chars();
    while let Some(c) = iter.next() {
        count += 1;
        if c == '"' || c == '\\' {
            count += 1;
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn actual_challenge() {
        let output = part_2_actual_challenge();        assert_eq!("2046", output);
    }
    #[test]
    fn example_test() {
        let r = part_2_example();
        assert_eq!("19", r);
    }
}
