use crate::day11::common;

fn part_2(input: &str, i: usize) -> String {
    let distances = common::calculate_distances(input, i);

    let sum: usize = distances.iter().sum();

    sum.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn actual_challenge() {
        let input = include_str!("input1.txt");
        let output = part_2(input, 1_000_000 - 1);
        
        assert_eq!("702770569197", output);
    }

    #[test]
    fn example_test_10() {
        let input = include_str!("input1_ex.txt"); // same
        let r = part_2(input, 9);
        assert_eq!("1030", r);
    }

    #[test]
    fn example_test_100() {
        let input = include_str!("input1_ex.txt"); // same
        let r = part_2(input, 99);
        assert_eq!("8410", r);
    }
}
