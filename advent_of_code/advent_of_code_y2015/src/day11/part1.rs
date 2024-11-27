use super::common::next_valid_password;

pub fn part_1_actual_challenge() -> String {
    let input = include_str!("input1.txt");
    part_1(input)
}

fn part_1(input: &str) -> String {
    let res = next_valid_password(input.trim().to_string().into_bytes());
    String::from_utf8(res).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn actual_challenge() {
        let output = part_1_actual_challenge();        assert_eq!("vzbxxyzz", output);
    }

    #[test]
    fn test_next_valid_password() {
        assert_eq!("abcdffaa", part_1("abcdefgh"));
        assert_eq!("ghjaabcc", part_1("ghijklmn"));
    }
}
