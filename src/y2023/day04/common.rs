pub fn get_card_infos(line: &str) -> (usize, Vec<u32>, Vec<u32>) {
    let mut parts = line.split(':');

    let card_id = parts
        .next()
        .and_then(|s| s.trim().split_whitespace().nth(1))
        .and_then(|s| s.parse::<usize>().ok())
        .unwrap_or_default();

    let card_info = parts.next().unwrap_or_default();

    let (winning_numbers, elf_numbers) = extract_numbers(card_info);

    (card_id, winning_numbers, elf_numbers)
}

fn extract_numbers(card_info: &str) -> (Vec<u32>, Vec<u32>) {
    let mut parts = card_info.split('|');

    let winning_numbers = get_numbers(&mut parts);
    let elf_numbers = get_numbers(&mut parts);

    (winning_numbers, elf_numbers)
}

fn get_numbers(parts: &mut dyn Iterator<Item = &str>) -> Vec<u32> {
    parts
        .next()
        .unwrap_or_default()
        .split_whitespace()
        .filter_map(|n| n.parse().ok())
        .collect()
}
