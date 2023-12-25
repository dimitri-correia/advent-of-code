use std::collections::HashMap;

use common::Colors;
use common::extract_game_info;

mod common;

fn main() {
    let input = include_str!("./input1.txt");
    let mut bag_possibilities = HashMap::new();
    bag_possibilities.insert(Colors::R, 12);
    bag_possibilities.insert(Colors::G, 13);
    bag_possibilities.insert(Colors::B, 14);
    let bag_possibilities = bag_possibilities;

    let output = part_1(input, &bag_possibilities);

    dbg!(output);
}

fn part_1(input: &str, bag_possibilities: &HashMap<Colors, u32>) -> String {
    let mut res = 0;
    'game: for line in input.lines() {
        let (game_id, subsets) = extract_game_info(line);
        dbg!(&game_id);
        for subset in subsets {
            dbg!(&subset);
            for (quantity, color) in subset {
                if match color {
                    Colors::R => bag_possibilities.get(&Colors::R).unwrap() < &quantity,
                    Colors::G => bag_possibilities.get(&Colors::G).unwrap() < &quantity,
                    Colors::B => bag_possibilities.get(&Colors::B).unwrap() < &quantity,
                } {
                    continue 'game;
                }
            }
        }
        res += game_id;
        dbg!(&res);
    }
    res.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("./input1_ex.txt");
        let mut bag_possibilities = HashMap::new();
        bag_possibilities.insert(Colors::R, 12);
        bag_possibilities.insert(Colors::G, 13);
        bag_possibilities.insert(Colors::B, 14);
        let bag_possibilities = bag_possibilities;

        let r = part_1(input, &bag_possibilities);

        assert_eq!("8", r);
    }
}
