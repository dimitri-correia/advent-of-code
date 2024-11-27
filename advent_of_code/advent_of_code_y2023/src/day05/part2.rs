use crate::day05::common::{get_maps, get_min_location_p_2};

pub fn part_2_example() -> String {
    let input = include_str!("input1_ex.txt");
    part_2(input)
}

pub fn part_2_actual_challenge() -> String {
    let input = include_str!("input1.txt");
    part_2(input)
}

fn part_2(input: &str) -> String {
    let input = get_maps(input);
    let res = get_min_location_p_2(input);

    res.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn actual_challenge() {
        let output = part_2_actual_challenge();        assert_eq!("60568880", output);
    }

    #[test]
    fn example_test() {
        let r = part_2_example();
        assert_eq!("46", r);
    }
}
