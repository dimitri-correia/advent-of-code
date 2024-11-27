use crate::day04::common::get_elves_ranges;

pub fn part_2_example() -> String {
    let input = include_str!("input1_ex.txt");
    part_2(input)
}

pub fn part_2_actual_challenge() -> String {
    let input = include_str!("input1.txt");
    part_2(input)
}

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
        let output = part_2_actual_challenge();        assert_eq!("804", output);
    }
    #[test]
    fn example_test() {
        let r = part_2_example();
        assert_eq!("4", r);
    }
}
