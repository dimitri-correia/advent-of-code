use crate::y2023::day11::common;

fn part_1(input: &str) -> String {
    let distances = common::calculate_distances(input, 1);

    let sum: usize = distances.iter().sum();

    sum.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn actual_challenge() {
        let input = include_str!("input1.txt");
        let output = part_1(input);
        dbg!(&output);
        assert_eq!("9974721", output);
    }

    #[test]
    fn example_test() {
        let input = include_str!("input1_ex.txt"); // same file
        let r = part_1(input);
        assert_eq!("374", r);
    }
}
