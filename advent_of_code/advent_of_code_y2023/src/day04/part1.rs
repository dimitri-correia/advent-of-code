use crate::day04::common;

pub fn part_1_example() -> String {
    let input = include_str!("input1_ex.txt");
    part_1(input)
}

pub fn part_1_actual_challenge() -> String {
    let input = include_str!("input1.txt");
    part_1(input)
}

fn part_1(input: &str) -> String {
    input
        .lines()
        .map(|line| calculate_card_points(line))
        .sum::<u32>()
        .to_string()
}

fn calculate_card_points(line: &str) -> u32 {
    let (_, winning_numbers, elf_numbers) = common::get_card_infos(line);

    elf_numbers
        .iter()
        .filter(|&elf_number| winning_numbers.contains(elf_number))
        .fold(0, |acc, _| if acc == 0 { 1 } else { acc * 2 })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn actual_challenge() {
        let output = part_1_actual_challenge();        
        assert_eq!("22897", output);
    }

    #[test]
    fn example_test() {
        let r = part_1_example();
        assert_eq!("13", r);
    }
}
