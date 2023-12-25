fn part_1(input: &str) -> String {
    let mut res = 0;

    input
        .split("\n")
        .filter(|line| !line.is_empty())
        .for_each(|line| {
            dbg!(line);
            res += line
                .chars()
                .find(|c| c.is_ascii_digit())
                .expect("here")
                .to_digit(10)
                .expect("here 1")
                * 10
                + line
                    .chars()
                    .rev()
                    .find(|c| c.is_ascii_digit())
                    .expect("here 2")
                    .to_digit(10)
                    .expect("here 3");
        });

    res.to_string()
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
