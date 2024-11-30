use super::common::get_number_possibilities;

pub fn part_2_example() -> String {
    let input = include_str!("input1_ex.txt");
    let total_amount = 25;
    part_2(input, total_amount)
}

pub fn part_2_actual_challenge() -> String {
    let input = include_str!("input1.txt");
    let total_amount = 150;
    part_2(input, total_amount)
}

fn part_2(input: &str, total_amount: u32) -> String {
    let possibilities = get_number_possibilities(input, total_amount);

    let (_, count) = possibilities
        .iter()
        .fold((usize::MAX, 0), |(min_val, count), &value| {
            if value < min_val {
                (value, 1)
            } else if value == min_val {
                (min_val, count + 1)
            } else {
                (min_val, count)
            }
        });

    count.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn actual_challenge() {
        let output = part_2_actual_challenge();
        assert_eq!("17", output);
    }

    #[test]
    fn example_test() {
        let r = part_2_example();
        assert_eq!("3", r);
    }
}
