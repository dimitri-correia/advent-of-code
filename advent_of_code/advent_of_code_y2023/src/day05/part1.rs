use crate::day05::common::{get_maps, get_min_location_p_1};

pub fn part_1_example() -> String {
    let input = include_str!("input1_ex.txt");
    part_1(input)
}

pub fn part_1_actual_challenge() -> String {
    let input = include_str!("input1.txt");
    part_1(input)
}

fn part_1(input: &str) -> String {
    let input = get_maps(input);

    let res = get_min_location_p_1(input);

    res.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn actual_challenge() {
        let output = part_1_actual_challenge();        
        assert_eq!("289863851", output);
    }

    #[test]
    fn example_test() {
        let r = part_1_example();
        assert_eq!("35", r);
    }
}
