use super::common::{compute_best_happiness, parse_input};

pub fn part_1_example() -> String {
    let input = include_str!("input1_ex.txt");
    part_1(input)
}

pub fn part_1_actual_challenge() -> String {
    let input = include_str!("input1.txt");
    part_1(input)
}

fn part_1(input: &str) -> String {
    let (arrangements, all_names) = parse_input(input);
    compute_best_happiness(all_names, arrangements)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn actual_challenge() {
        let output = part_1_actual_challenge();
        assert_eq!("664", output);
    }

    #[test]
    fn example_test() {
        let r = part_1_example();
        assert_eq!("330", r);
    }
}
