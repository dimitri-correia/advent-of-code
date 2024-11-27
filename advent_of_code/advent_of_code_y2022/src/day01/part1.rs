use crate::day01::common;

pub fn part_1_example() -> String {
    let input = include_str!("input1_ex.txt");
    part_1(input)
}

pub fn part_1_actual_challenge() -> String {
    let input = include_str!("input1.txt");
    part_1(input)
}

fn part_1(input: &str) -> String {
    let items_calo_per_elves = common::get_calo_per_elf(input);
    let max_calo = items_calo_per_elves.iter().max().unwrap();
    max_calo.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn actual_challenge() {
        let output = part_1_actual_challenge();        
        assert_eq!("72478", output);
    }

    #[test]
    fn example_test() {
        let r = part_1_example();
        assert_eq!("24000", r);
    }
}
