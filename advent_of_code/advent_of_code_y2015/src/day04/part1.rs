use super::common::find_hash_starting_with_pre;

pub fn part_1_example() -> String {
    let input = include_str!("input1_ex.txt");
    part_1(input)
}

pub fn part_1_actual_challenge() -> String {
    let input = include_str!("input1.txt");
    part_1(input)
}

fn part_1(input: &str) -> String {
    find_hash_starting_with_pre(input.trim(), "00000")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn actual_challenge() {
        let output = part_1_actual_challenge();        
        assert_eq!("282749", output);
    }

    #[test]
    fn example_test() {
        let r = part_1_example();
        assert_eq!("1048970", r);
    }
}
