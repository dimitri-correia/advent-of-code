pub fn get_elves_ranges(l: &str) -> ((usize, usize), (usize, usize)) {
    let (elf_a, elf_b) = l.split_once(',').unwrap();
    (get_elf_range(elf_a), get_elf_range(elf_b))
}

fn get_elf_range(elf_a: &str) -> (usize, usize) {
    let (elf_start, elf_end) = elf_a.split_once('-').unwrap();
    (elf_start.parse().unwrap(), elf_end.parse().unwrap())
}
