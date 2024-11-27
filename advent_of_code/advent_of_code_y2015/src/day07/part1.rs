use std::collections::HashMap;

use super::common::{follow_wire, parse_input};

pub fn part_1_example() -> Vec<String> {
    let input = include_str!("input1_ex.txt");
    vec![
        part_1(input, "d"),
        part_1(input, "e"),
        part_1(input, "f"),
        part_1(input, "g"),
        part_1(input, "h"),
        part_1(input, "i"),
        part_1(input, "x"),
        part_1(input, "y"),
    ]
}

pub fn part_1_actual_challenge() -> String {
    let input = include_str!("input1.txt");
    part_1(input, "a")
}

fn part_1(input: &str, starting_wire: &str) -> String {
    let wires = parse_input(input);
    let mut cache: HashMap<String, u16> = HashMap::new();
    follow_wire(starting_wire, &wires, &mut cache).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn actual_challenge() {
        let output = part_1_actual_challenge();        
        assert_eq!("46065", output);
    }

    #[test]
    fn example_test() {
        let r = part_1_example();
        assert_eq!(
            vec!["72", "507", "492", "114", "65412", "65079", "123", "456"],
            r
        );
    }
}
