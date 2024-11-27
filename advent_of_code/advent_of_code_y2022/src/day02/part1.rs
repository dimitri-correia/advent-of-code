use crate::day02::common::GameItems;
use std::str::FromStr;

pub fn part_1_example() -> String {
    let input = include_str!("input1_ex.txt");
    part_1(input)
}

pub fn part_1_actual_challenge() -> String {
    let input = include_str!("input1.txt");
    part_1(input)
}

fn part_1(input: &str) -> String {
    input
        .lines()
        .fold(0, |acc, line| {
            let (opponent, me) = get_move(line);
            let result = me.get_round_result(&opponent);
            acc + result as u32 + me as u32
        })
        .to_string()
}

fn get_move(line: &str) -> (GameItems, GameItems) {
    let mut round = line.split_whitespace();
    let opponent = GameItems::from_str(round.next().unwrap()).unwrap();
    let me = GameItems::from_str(round.next().unwrap()).unwrap();
    (opponent, me)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn actual_challenge() {
        let output = part_1_actual_challenge();        
        assert_eq!("17189", output);
    }

    #[test]
    fn example_test() {
        let r = part_1_example();
        assert_eq!("15", r);
    }
}
