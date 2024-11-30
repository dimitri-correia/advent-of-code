use super::common::get_number_possibilities;

pub fn part_1_example() -> String {
    let input = include_str!("input1_ex.txt");
    let total_amount = 25;
    part_1(input, total_amount)
}

pub fn part_1_actual_challenge() -> String {
    let input = include_str!("input1.txt");
    let total_amount = 150;
    part_1(input, total_amount)
}

fn part_1(input: &str, total_amount: u32) -> String {
    let possibilities = get_number_possibilities(input, total_amount);

    possibilities.len().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn actual_challenge() {
        let output = part_1_actual_challenge();
        assert_eq!("1638", output);
    }

    #[test]
    fn example_test() {
        let r = part_1_example();
        assert_eq!("4", r);
    }
}
