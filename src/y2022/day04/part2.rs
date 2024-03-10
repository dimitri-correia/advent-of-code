use crate::y2022::day04::common::get_elves_ranges;

fn part_2(input: &str) -> String {
    input
        .lines()
        .filter(|&l| {
            let (elf_a, elf_b) = get_elves_ranges(l);
            overlap(elf_a, elf_b)
        })
        .count()
        .to_string()
}

fn overlap(a: (usize, usize), b: (usize, usize)) -> bool {
    b.0 <= a.1 && b.1 >= a.0
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
