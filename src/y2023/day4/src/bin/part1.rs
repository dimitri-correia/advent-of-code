mod common;

fn main() {
    let input = include_str!("input1.txt");
    let output = part_1(input);
    dbg!(output);
}

fn part_1(input: &str) -> String {
    let mut res = 0;
    for line in input.lines() {
        let (_, winning_numbers, elf_numbers) = common::get_card_infos(line);
        //dbg!((&winning_numbers, &elf_numbers));

        let mut card_points = 0;
        for elf_number in &elf_numbers {
            if winning_numbers.contains(elf_number) {
                if card_points == 0 {
                    card_points = 1;
                } else {
                    card_points *= 2;
                }
            }
        }
        dbg!(card_points);
        res += card_points;
    }
    res.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("input1_ex.txt");
        let r = part_1(input);
        assert_eq!("13", r);
    }
}
