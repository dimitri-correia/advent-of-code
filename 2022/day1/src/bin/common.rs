pub fn get_calo_per_elf(input: &str) -> Vec<u32> {
    let mut items_calo_per_elves = vec![];
    let mut current_elf = 0;
    for line in input.lines() {
        if line.trim().is_empty() {
            items_calo_per_elves.push(current_elf);
            current_elf = 0;
            continue;
        }
        current_elf += line.parse::<u32>().unwrap();
    }
    items_calo_per_elves.push(current_elf);
    items_calo_per_elves
}
