use super::part1::part_1;

pub fn part_2_actual_challenge() -> String {
    let mut input = include_str!("input1.txt").to_string();
    for _ in 0..50 {
        input = part_1(input);
    }
    input.len().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn actual_challenge() {
        let output = part_2_actual_challenge();
        assert_eq!("", output);
    }
}
