fn part_2(input: &str) -> String {
    "".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn actual_challenge() {
        let input = include_str!("input1.txt");
        let output = part_2(input);
        dbg!(&output);
        assert_eq!("", output);
    }
    #[test]
    fn example_test() {
        let input = include_str!("input1_ex.txt");
        let r = part_2(input);
        assert_eq!("", r);
    }
}
