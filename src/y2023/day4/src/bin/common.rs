use std::str::Split;

pub fn get_card_infos(line: &str) -> (usize, Vec<u32>, Vec<u32>) {
    let mut parts = line.split(':');
    let mut card_id_parts = parts.next().unwrap().split_whitespace();
    card_id_parts.next();
    let card_id = card_id_parts.next().unwrap().parse::<usize>().unwrap();
    dbg!(card_id);
    let card_info = parts.next().unwrap();
    parts = card_info.split('|');
    let winning_numbers: Vec<u32> = get_numbers(&mut parts);
    let elf_numbers: Vec<u32> = get_numbers(&mut parts);
    (card_id, winning_numbers, elf_numbers)
}

fn get_numbers(parts: &mut Split<char>) -> Vec<u32> {
    parts
        .next()
        .unwrap()
        .split_whitespace()
        .map(|n| n.parse::<u32>().unwrap())
        .collect()
}
