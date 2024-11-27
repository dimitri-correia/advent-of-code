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
        let output = part_1_actual_challenge();        
        assert_eq!("8105", output);
    }

    #[test]
    fn example_test() {
        let r = part_1_example();
        assert_eq!("157", r);
    }
}
