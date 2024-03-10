fn part_1(input: &str) -> String {
    input
        .lines()
        .fold(0, |acc, line| {
            let line_bytes = line.as_bytes();
            let size = line_bytes.len();
            let idx_common = (0..size / 2)
                .find(|&i| line_bytes[size / 2..].contains(&line_bytes[i]))
                .unwrap();

            let common = (if line_bytes[idx_common].is_ascii_lowercase() {
                line_bytes[idx_common] - b'a' + 1
            } else {
                // uppercase
                line_bytes[idx_common] - b'A' + 27
            }) as u32;

            acc + common
        })
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn actual_challenge() {
        let input = include_str!("input1.txt");
        let output = part_1(input);
        dbg!(&output);
        assert_eq!("8105", output);
    }

    #[test]
    fn example_test() {
        let input = include_str!("input1_ex.txt");
        let r = part_1(input);
        assert_eq!("157", r);
    }
}
