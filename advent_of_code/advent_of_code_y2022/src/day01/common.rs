pub fn get_calo_per_elf(input: &str) -> Vec<u32> {
    input
        .split("\n\n")
        .map(|group| {
            group
                .lines()
                .map(|line| line.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .collect()
}
