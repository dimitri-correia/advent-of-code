pub fn part_2_example() -> String {
    let input = include_str!("input1_ex.txt");
    part_2(input)
}

pub fn part_2_actual_challenge() -> String {
    let input = include_str!("input1.txt");
    part_2(input)
}

fn part_2(input: &str) -> String {
    let mut pos = 0;
    for (i, c) in input.char_indices() {
        match c {
            '(' => pos += 1,
            ')' => pos -= 1,
            _ => unreachable!(),
        }
        if pos == -1 {
            return (i + 1).to_string();
        }
    }
    "".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn actual_challenge() {
        let output = part_2_actual_challenge();        assert_eq!("1795", output);
    }
    #[test]
    fn example_test() {
        let r = part_2_example();
        assert_eq!("1", r);
    }
}
