use crate::day09::common::get_compute;

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
        .map(|line| {
            let compute = get_compute(line);
            compute.iter().map(|row| row.last().unwrap()).sum::<isize>()
        })
        .sum::<isize>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn actual_challenge() {
        let output = part_1_actual_challenge();        
        assert_eq!("1696140818", output);
    }

    #[test]
    fn example_test() {
        let r = part_1_example();
        assert_eq!("114", r);
    }
}
