fn part_2(input: &str) -> String {
    input
        .lines()
        .filter(|l| {
            let (elf_a, elf_b) = l.split_once(',').unwrap();
            let elf_a = get_elf_range(elf_a);
            let elf_b = get_elf_range(elf_b);
            overlap(elf_a, elf_b)
        })
        .count()
        .to_string()
}

fn overlap(a: (usize, usize), b: (usize, usize)) -> bool {
    b.0 <= a.1 && b.1 >= a.0
}

fn get_elf_range(elf_a: &str) -> (usize, usize) {
    let (elf_start, elf_end) = elf_a.split_once('-').unwrap();
    (elf_start.parse().unwrap(), elf_end.parse().unwrap())
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn actual_challenge() {
        let input = include_str!("input1.txt");
        let output = part_2(input);
        dbg!(&output);
        assert_eq!("804", output);
    }
    #[test]
    fn example_test() {
        let input = include_str!("input1_ex.txt");
        let r = part_2(input);
        assert_eq!("4", r);
    }
}
