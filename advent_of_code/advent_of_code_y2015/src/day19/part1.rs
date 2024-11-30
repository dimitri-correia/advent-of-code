use std::collections::HashSet;

use super::common::parse_input;

pub fn part_1_example() -> String {
    let input = include_str!("input1_ex.txt");
    part_1(input)
}

pub fn part_1_actual_challenge() -> String {
    let input = include_str!("input1.txt");
    part_1(input)
}

fn part_1(input: &str) -> String {
    let (start_molecule, replacements) = parse_input(input);
    let mut molecules = HashSet::new();
    for (from, to) in replacements.iter() {
        let mut idx = 0;
        while let Some(index) = start_molecule.split_at(idx).1.find(from) {
            let (left, right) = start_molecule.split_at(idx + index);
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
        let output = part_1_actual_challenge();
        assert_eq!("509", output);
    }

    #[test]
    fn example_test() {
        let r = part_1_example();
        assert_eq!("4", r);
    }
}
