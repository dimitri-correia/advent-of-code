use crate::y2023::day04::common;

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
        let input = include_str!("input1.txt");
        let output = part_1(input);
        dbg!(&output);
        assert_eq!("22897", output);
    }

    #[test]
    fn example_test() {
        let input = include_str!("input1_ex.txt"); // same file
        let r = part_1(input);
        assert_eq!("13", r);
    }
}
