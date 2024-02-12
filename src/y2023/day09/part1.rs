use crate::y2023::day09::common::get_compute;

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
        let input = include_str!("input1.txt");
        let output = part_1(input);
        dbg!(&output);
        assert_eq!("1696140818", output);
    }

    #[test]
    fn example_test() {
        let input = include_str!("input1_ex.txt");
        let r = part_1(input);
        assert_eq!("114", r);
    }
}
