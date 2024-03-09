use crate::y2022::day02::common::{GameItems, RoundResult};
use std::str::FromStr;

fn part_2(input: &str) -> String {
    input
        .lines()
        .fold(0, |acc, line| {
            let (opponent_move, expected_result) = get_move(line);
            let my_move = opponent_move.get_move_for_result(&expected_result);
            acc + expected_result as u32 + my_move as u32
        })
        .to_string()
}

fn get_move(line: &str) -> (GameItems, RoundResult) {
    let mut round = line.split_whitespace();
    let opponent_move = GameItems::from_str(round.next().unwrap()).unwrap();
    let expected_result = RoundResult::from_str(round.next().unwrap()).unwrap();
    (opponent_move, expected_result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn actual_challenge() {
        let input = include_str!("input1.txt");
        let output = part_2(input);
        dbg!(&output);
        assert_eq!("13490", output);
    }
    #[test]
    fn example_test() {
        let input = include_str!("input1_ex.txt");
        let r = part_2(input);
        assert_eq!("12", r);
    }
}
