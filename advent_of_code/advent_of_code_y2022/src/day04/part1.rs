use crate::day04::common::get_elves_ranges;

pub fn part_1_example() -> String {
    let input = include_str!("input1_ex.txt");
    part_1(input)
}

pub fn part_1_actual_challenge() -> String {
    let input = include_str!("input1.txt");
    part_1(input)
}

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
        let output = part_1_actual_challenge();        
        assert_eq!("424", output);
    }

    #[test]
    fn example_test() {
        let r = part_1_example();
        assert_eq!("2", r);
    }
}
