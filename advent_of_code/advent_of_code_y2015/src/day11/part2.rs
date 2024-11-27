use super::common::{inc_password, next_valid_password};

pub fn part_2_actual_challenge() -> String {
    let input = include_str!("input1.txt");
    part_2(input)
}

fn part_2(input: &str) -> String {
    let mut second_pass = next_valid_password(input.trim().to_string().into_bytes());
    inc_password(&mut second_pass);
    let res = next_valid_password(second_pass);
    String::from_utf8(res).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn actual_challenge() {
        let output = part_2_actual_challenge();
        assert_eq!("vzcaabcc", output);
    }
}
