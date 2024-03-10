use crate::y2022::day04::common::get_elves_ranges;

fn part_1(input: &str) -> String {
    input
        .lines()
        .filter(|&l| {
            let (elf_a, elf_b) = get_elves_ranges(l);
            contains(elf_a, elf_b)
        })
        .count()
        .to_string()
}

fn contains(a: (usize, usize), b: (usize, usize)) -> bool {
    b.0 >= a.0 && b.1 <= a.1 || a.0 >= b.0 && a.1 <= b.1
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
