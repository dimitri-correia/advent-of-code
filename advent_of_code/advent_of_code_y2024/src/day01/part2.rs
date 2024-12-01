use std::collections::HashMap;

use super::common::parse_input;

pub fn part_2_example() -> String {
    let input = include_str!("input1_ex.txt");
    part_2(input)
}

pub fn part_2_actual_challenge() -> String {
    let input = include_str!("input1.txt");
    part_2(input)
}

fn part_2(input: &str) -> String {
    let (list_one, list_two) = parse_input(input);
    let counter_one: HashMap<i32, i32> = list_one.iter().fold(HashMap::new(), |mut acc, &x| {
        *acc.entry(x).or_insert(0) += 1;
        acc
    });
    let counter_two: HashMap<i32, i32> = list_two.iter().fold(HashMap::new(), |mut acc, &x| {
        *acc.entry(x).or_insert(0) += 1;
        acc
    });
    let mut result = 0;
    for k in counter_one.keys() {
        result += k * counter_one.get(k).unwrap_or(&0) * counter_two.get(k).unwrap_or(&0);
    }

    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn actual_challenge() {
        let output = part_2_actual_challenge();
        assert_eq!("18650129", output);
    }
    #[test]
    fn example_test() {
        let r = part_2_example();
        assert_eq!("31", r);
    }
}
