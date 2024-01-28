use crate::y2023::day10::common::get_path;

fn part_1(input: &str) -> String {
    let (pass, _) = get_path(input);

    (pass.len() / 2).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn actual_challenge() {
        let input = include_str!("input1.txt");
        let output = part_1(input);
        dbg!(&output);
        assert_eq!("7145", output);
    }

    #[test]
    fn example_test_a() {
        let input = include_str!("input1_ex.txt");
        let r = part_1(input);
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
