fn main() {
    let input = include_str!("./input1.txt");
    let output = part_1(input);
    dbg!(output);
}

fn part_1(input: &str) -> String {
    let mut res = 0;
    for line in input.lines() {
        let (winning_numbers, elf_numbers) = get_card_infos(line);
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

fn get_card_infos(line: &str) -> (Vec<u32>, Vec<u32>) {
    let mut parts = line.split(':');
    let _card_id_str = parts.next().unwrap();
    dbg!(_card_id_str);
    let card_info = parts.next().unwrap();
    parts = card_info.split('|');
    let winning_numbers: Vec<u32> = parts
        .next()
        .unwrap()
        .split_whitespace()
        .map(|n| n.parse::<u32>().unwrap())
        .collect();
    let elf_numbers: Vec<u32> = parts
        .next()
        .unwrap()
        .split_whitespace()
        .map(|n| n.parse::<u32>().unwrap())
        .collect();
    (winning_numbers, elf_numbers)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("./input1_ex.txt");
        let r = part_1(input);
        assert_eq!("13", r);
    }
}
