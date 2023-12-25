use std::str::FromStr;

use common::GameItems;
use common::GetRoundResult;

mod common;

fn main() {
    let input = include_str!("./input1.txt");
    let output = part_1(input);
    dbg!(output);
}

fn part_1(input: &str) -> String {
    dbg!(input);
    let mut res = 0;
    for line in input.lines() {
        let (opponent, me) = get_move(line);
        dbg!(&opponent, &me);
        let result = me.get_round_result(&opponent);
        dbg!(&result);
        res += result as u32 + me as u32;
    }
    res.to_string()
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
    fn it_works() {
        let input = include_str!("./input1_ex.txt");
        let r = part_1(input);
        assert_eq!("15", r);
    }
}
