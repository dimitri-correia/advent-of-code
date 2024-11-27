use crate::day10::common::get_path;

pub fn part_1_example() -> String {
    let input = include_str!("input1_ex.txt");
    part_1(input)
}

pub fn part_1_actual_challenge() -> String {
    let input = include_str!("input1.txt");
    part_1(input)
}

fn part_1(input: &str) -> String {
    let (pass, _) = get_path(input);

    (pass.len() / 2).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn actual_challenge() {
        let output = part_1_actual_challenge();        
        assert_eq!("7145", output);
    }

    #[test]
    fn example_test_a() {
        let r = part_1_example();
        assert_eq!("4", r);
    }

    #[test]
    fn example_test_b() {
        let input = include_str!("input1_ex2.txt");
        let r = part_1(input);
        assert_eq!("4", r);
    }

    #[test]
    fn example_test_c() {
        let input = include_str!("input1_ex3.txt");
        let r = part_1(input);
        assert_eq!("8", r);
    }

    #[test]
    fn example_test_d() {
        let input = include_str!("input1_ex4.txt");
        let r = part_1(input);
        assert_eq!("8", r);
    }
}
