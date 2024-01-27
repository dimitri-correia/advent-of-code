use crate::y2023::day05::common::{get_maps, get_min_location_p_1};

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
        let input = include_str!("input1.txt");
        let output = part_1(input);
        dbg!(&output);
        assert_eq!("289863851", output);
    }

    #[test]
    fn example_test() {
        let input = include_str!("input1_ex.txt"); // same file
        let r = part_1(input);
        assert_eq!("35", r);
    }
}
