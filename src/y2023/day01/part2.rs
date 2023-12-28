fn part_2(input: &str) -> String {
    input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            let first_digit = get_first_digit(line);
            let last_digit = get_last_digit(line);

            10 * first_digit + last_digit
        })
        .sum::<u32>()
        .to_string()
}

fn get_first_digit(s: &str) -> u32 {
    s.chars()
        .enumerate()
        .find_map(|(idx, c)| {
            let rest_of_line: String = s[idx..].to_string();
            get_digit(c, rest_of_line)
        })
        .unwrap()
}

fn get_last_digit(s: &str) -> u32 {
    s.chars()
        .rev()
        .enumerate()
        .find_map(|(idx, c)| {
            let rest_of_line: String = s[s.len() - idx..].to_string();
            get_digit(c, rest_of_line)
        })
        .unwrap()
}

fn get_digit(c: char, rest_of_line: String) -> Option<u32> {
    match rest_of_line {
        s if s.starts_with("one") => Some(1),
        s if s.starts_with("two") => Some(2),
        s if s.starts_with("three") => Some(3),
        s if s.starts_with("four") => Some(4),
        s if s.starts_with("five") => Some(5),
        s if s.starts_with("six") => Some(6),
        s if s.starts_with("seven") => Some(7),
        s if s.starts_with("eight") => Some(8),
        s if s.starts_with("nine") => Some(9),
        _ if c.is_ascii_digit() => Some(c.to_digit(10).unwrap()),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn actual_challenge() {
        let input = include_str!("input1.txt"); //same
        let output = part_2(input);
        dbg!(&output);
        assert_eq!("53312", output);
    }
    #[test]
    fn example_test() {
        let input = include_str!("input2_ex.txt");
        let r = part_2(input);
        assert_eq!("281", r);
    }
}
