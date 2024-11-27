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
        .filter(|l| {
            l.chars().filter(|c| "aeiou".contains(*c)).count() >= 3
                && l.chars().zip(l.chars().skip(1)).any(|(a, b)| a == b)
                && !l.contains("ab")
                && !l.contains("cd")
                && !l.contains("pq")
                && !l.contains("xy")
        })
        .count()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn actual_challenge() {
        let output = part_1_actual_challenge();        
        assert_eq!("238", output);
    }

    #[test]
    fn example_test() {
        let r = part_1_example();
        assert_eq!("2", r);
    }
}
