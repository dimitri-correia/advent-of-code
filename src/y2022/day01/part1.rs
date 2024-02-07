use crate::y2022::day01::common;

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
        let input = include_str!("input1.txt");
        let output = part_1(input);
        dbg!(&output);
        assert_eq!("72478", output);
    }

    #[test]
    fn example_test() {
        let input = include_str!("input1_ex.txt");
        let r = part_1(input);
        assert_eq!("24000", r);
    }
}
