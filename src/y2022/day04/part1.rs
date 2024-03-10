fn part_1(input: &str) -> String {
    input
        .lines()
        .filter(|l| {
            let (elf_a, elf_b) = l.split_once(',').unwrap();
            let elf_a = get_elf_range(elf_a);
            let elf_b = get_elf_range(elf_b);
            contains(elf_a, elf_b)
        })
        .count()
        .to_string()
}

fn contains(a: (usize, usize), b: (usize, usize)) -> bool {
    b.0 >= a.0 && b.1 <= a.1 || a.0 >= b.0 && a.1 <= b.1
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
        let output = part_1(input);
        dbg!(&output);
        assert_eq!("424", output);
    }

    #[test]
    fn example_test() {
        let input = include_str!("input1_ex.txt");
        let r = part_1(input);
        assert_eq!("2", r);
    }
}
