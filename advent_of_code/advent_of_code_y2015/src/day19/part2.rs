use std::collections::HashSet;

use super::common::parse_input;

pub fn part_2_example() -> String {
    let input = include_str!("input2_ex.txt");
    part_2(input)
}

pub fn part_2_actual_challenge() -> String {
    let input = include_str!("input1.txt");
    part_2(input)
}

fn part_2(input: &str) -> String {
    let (needed_molecule, replacements) = parse_input(input);
    let mut molecules = HashSet::new();
    for (from, to) in replacements.iter() {
        let mut idx = 0;
        while let Some(index) = needed_molecule.split_at(idx).1.find(from) {
            let (left, right) = needed_molecule.split_at(idx + index);
            let (_, right) = right.split_at(from.len());
            let new_molecule = format!("{}{}{}", left, to, right);
            molecules.insert(new_molecule);
            idx = idx + index + 1;
        }
    }
    molecules.len().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn actual_challenge() {
        let output = part_2_actual_challenge();
        assert_eq!("", output);
    }
    #[test]
    fn example_test() {
        let r = part_2_example();
        assert_eq!("6", r);
    }
}
