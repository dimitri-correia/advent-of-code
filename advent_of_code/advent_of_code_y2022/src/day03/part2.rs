use itertools::Itertools;

pub fn part_2_example() -> String {
    let input = include_str!("input1_ex.txt");
    part_2(input)
}

pub fn part_2_actual_challenge() -> String {
    let input = include_str!("input1.txt");
    part_2(input)
}

fn part_2(input: &str) -> String {
    let mut lines = input.lines();
    let mut r = 0;
    while let Some(line_a) = lines.next() {
        let line_b = lines.next().unwrap();
        let line_c = lines.next().unwrap();
        let c = line_a
            .chars()
            .find(|c| line_b.chars().contains(c) && line_c.chars().contains(c))
            .unwrap();
        let common = (if c.is_ascii_lowercase() {
            c as u8 - b'a' + 1
        } else {
            // uppercase
            c as u8 - b'A' + 27
        }) as u32;
        r += common;
    }

    r.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn actual_challenge() {
        let output = part_2_actual_challenge();        assert_eq!("2363", output);
    }
    #[test]
    fn example_test() {
        let r = part_2_example();
        assert_eq!("70", r);
    }
}
