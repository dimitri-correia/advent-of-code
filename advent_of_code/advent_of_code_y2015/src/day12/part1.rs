pub fn part_1_example() -> String {
    let input = include_str!("input1_ex.txt");
    part_1(input)
}

pub fn part_1_actual_challenge() -> String {
    let input = include_str!("input1.txt");
    part_1(input)
}

fn part_1(input: &str) -> String {
    let mut chars = input.chars();
    let mut sum = 0;
    let mut curr = 0;
    let mut sign = 1;
    loop {
        let c = chars.next();
        if c.is_none() {
            break;
        }
        let c = c.unwrap();
        match c {
            '-' => {
                sign = -1;
            }
            '0'..='9' => {
                curr = curr * 10 + c.to_digit(10).unwrap() as i32;
            }
            _ => {
                sum += curr * sign;
                curr = 0;
                sign = 1;
                continue;
            }
        }
    }
    sum.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn actual_challenge() {
        let output = part_1_actual_challenge();
        assert_eq!("191164", output);
    }

    #[test]
    fn example_test() {
        let r = part_1_example();
        assert_eq!("3", r);
    }
}
