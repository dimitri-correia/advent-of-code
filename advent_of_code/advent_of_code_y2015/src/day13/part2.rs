use super::common::{compute_best_happiness, parse_input};

pub fn part_2_example() -> String {
    let input = include_str!("input1_ex.txt");
    part_2(input)
}

pub fn part_2_actual_challenge() -> String {
    let input = include_str!("input1.txt");
    part_2(input)
}

fn part_2(input: &str) -> String {
    let (arrangements, all_names) = parse_input(input);
    let mut all_names = all_names;
    let me = "AAAAAAA";
    all_names.push(me);
    let mut arrangements = arrangements;
    all_names.iter().for_each(|name| {
        arrangements.insert(format!("{}, {}", me, name), 0);
    });
    compute_best_happiness(all_names, arrangements)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn actual_challenge() {
        let output = part_2_actual_challenge();        assert_eq!("640", output);
    }
    #[test]
    fn example_test() {
        let r = part_2_example();
        assert_eq!("286", r);
    }
}
