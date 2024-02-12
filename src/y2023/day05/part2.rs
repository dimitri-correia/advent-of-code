use crate::y2023::day05::common::{get_maps, get_min_location_p_2};

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
        let input = include_str!("input1.txt");
        let output = part_2(input);
        dbg!(&output);
        assert_eq!("60568880", output);
    }

    #[test]
    fn example_test() {
        let input = include_str!("input1_ex.txt");
        let r = part_2(input);
        assert_eq!("46", r);
    }
}
