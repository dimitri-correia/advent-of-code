use std::str::FromStr;

use common::{GameItems, GetRoundResult, RoundResult};

mod common;

fn main() {
    let input = include_str!("input1.txt"); //same
    let output = part_2(input);
    dbg!(output);
}

fn part_2(input: &str) -> String {
    dbg!(input);
    let mut res = 0;
    for line in input.lines() {
        let (opponent_move, expected_result) = get_move(line);
        dbg!((&opponent_move, &expected_result));
        let my_move = opponent_move.get_move_for_result(&expected_result);
        dbg!(&my_move);
        res += expected_result as u32 + my_move as u32;
        dbg!(res);
    }
    res.to_string()
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
    fn it_works() {
        let input = include_str!("input1_ex.txt"); // same
        let r = part_2(input);
        assert_eq!("12", r);
    }
}
