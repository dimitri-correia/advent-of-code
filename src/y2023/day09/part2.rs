use crate::y2023::day09::common::get_compute;

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
        let input = include_str!("input1.txt");
        let output = part_2(input);
        dbg!(&output);
        assert_eq!("1152", output);
    }

    #[test]
    fn example_test() {
        let input = include_str!("input1_ex.txt"); // same file
        let r = part_2(input);
        assert_eq!("2", r);
    }
}
