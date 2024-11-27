use std::vec;

pub fn part_1_example() -> String {
    let input = include_str!("input1_ex.txt");
    part_1(input.to_string())
}

pub fn part_1_actual_challenge() -> String {
    let mut input = include_str!("input1.txt").to_string();
    for _ in 0..40 {
        input = part_1(input);
    }
    input.len().to_string()
}

pub fn part_1(input: String) -> String {
    let mut groups: Vec<(char, usize)> = vec![];
    input.trim().chars().fold('a', |last_c, c| {
        if c == last_c {
            let last_group = groups.last_mut().unwrap();
            last_group.1 += 1;
        } else {
            groups.push((c, 1));
        }
        c
    });

    groups
        .iter()
        .map(|(c, count)| format!("{}{}", count, c))
        .collect::<Vec<String>>()
        .join("")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn actual_challenge() {
        let output = part_1_actual_challenge();        assert_eq!("492982", output);
    }

    #[test]
    fn example_test() {
        let r = part_1_example();
        assert_eq!("312211", r);
    }
}
