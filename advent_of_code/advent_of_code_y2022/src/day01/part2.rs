use crate::day01::common;

pub fn part_2_example() -> String {
    let input = include_str!("input1_ex.txt");
    part_2(input)
}

pub fn part_2_actual_challenge() -> String {
    let input = include_str!("input1.txt");
    part_2(input)
}

fn part_2(input: &str) -> String {
    let mut items_calo_per_elves = common::get_calo_per_elf(input);
    items_calo_per_elves.sort_by(|a, b| b.cmp(a));
    let max_calo: u32 = items_calo_per_elves.iter().take(3).sum();
    max_calo.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn actual_challenge() {
        let output = part_2_actual_challenge();        assert_eq!("210367", output);
    }
    #[test]
    fn example_test() {
        let r = part_2_example();
        assert_eq!("45000", r);
    }
}
