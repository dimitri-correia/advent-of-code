use super::common::{follow_wire, parse_input};
use std::collections::HashMap;

pub fn part_2_actual_challenge() -> String {
    let input = include_str!("input1.txt");
    let starting_wire = "a";
    let second_wire = "b";
    part_2(input, starting_wire, second_wire)
}

fn part_2(input: &str, starting_wire: &str, second_wire: &str) -> String {
    let mut wires = parse_input(input);
    let mut cache: HashMap<String, u16> = HashMap::new();
    let first = follow_wire(starting_wire, &wires, &mut cache).to_string();
    wires.insert(second_wire, &first);
    let mut cache: HashMap<String, u16> = HashMap::new();
    follow_wire(starting_wire, &wires, &mut cache).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn actual_challenge() {
        let output = part_2_actual_challenge();        assert_eq!("14134", output);
    }
}
