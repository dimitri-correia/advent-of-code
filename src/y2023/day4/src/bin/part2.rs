mod common;

fn main() {
    let input = include_str!("input1.txt"); //same
    let output = part_2(input);
    dbg!(output);
}

fn part_2(input: &str) -> String {
    let mut cards_amount = vec![1; input.lines().count()];
    dbg!(&cards_amount);
    for line in input.lines() {
        let (mut card_id, winning_numbers, elf_numbers) = common::get_card_infos(line);
        let number_win = elf_numbers
            .iter()
            .filter(|&elf_number| winning_numbers.contains(elf_number))
            .count();
        card_id -= 1; // vec idx start at 0
        for i in 1..number_win + 1 {
            cards_amount[i + card_id] += cards_amount[card_id];
        }
        dbg!(&cards_amount);
    }
    cards_amount.iter().sum::<usize>().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("input1_ex.txt"); // same
        let r = part_2(input);
        assert_eq!("30", r);
    }
}
