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
        .split(',')
        .map(|hash| {
            hash.chars()
                .fold(0, |acc, next_char| (acc + (next_char as usize)) * 17 % 256)
        })
        .sum::<usize>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn actual_challenge() {
        let output = part_1_actual_challenge();        
        assert_eq!("517551", output);
    }

    #[test]
    fn example_test() {
        let r = part_1_example();
        assert_eq!("1320", r);
    }
}
