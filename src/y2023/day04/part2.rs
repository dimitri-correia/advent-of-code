use crate::y2023::day04::common;

fn part_2(input: &str) -> String {
    let cards_amount: Vec<usize> = input
        .lines()
        .fold(vec![1; input.lines().count()], |acc, line| {
            calculate_card_amounts(acc, line)
        });

    cards_amount.iter().sum::<usize>().to_string()
}

fn calculate_card_amounts(cards_amount: Vec<usize>, line: &str) -> Vec<usize> {
    let (mut card_id, winning_numbers, elf_numbers) = common::get_card_infos(line);
    let number_win = elf_numbers
        .iter()
        .filter(|&elf_number| winning_numbers.contains(elf_number))
        .count();
    card_id -= 1; // vec idx starts at 0

    (1..=number_win).fold(cards_amount, |mut acc, i| {
        acc[i + card_id] += acc[card_id];
        acc
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn actual_challenge() {
        let input = include_str!("input1.txt");
        let output = part_2(input);
        dbg!(&output);
        assert_eq!("5095824", output);
    }

    #[test]
    fn example_test() {
        let input = include_str!("input1_ex.txt");
        let r = part_2(input);
        assert_eq!("30", r);
    }
}
