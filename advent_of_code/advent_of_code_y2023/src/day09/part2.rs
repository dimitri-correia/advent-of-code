use crate::day09::common::get_compute;

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
        .map(|line| {
            let compute = get_compute(line);
            compute
                .iter()
                .rev()
                .fold(0, |acc, l| l.first().map_or(acc, |&first| first - acc))
        })
        .sum::<isize>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn actual_challenge() {
        let output = part_2_actual_challenge();        assert_eq!("1152", output);
    }

    #[test]
    fn example_test() {
        let r = part_2_example();
        assert_eq!("2", r);
    }
}
