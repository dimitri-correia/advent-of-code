use std::cmp::max;
use std::collections::HashMap;

use common::extract_game_info;
use common::Colors;

mod common;

fn main() {
    let input = include_str!("input1.txt"); // same file
    let output = part_2(input);
    dbg!(output);
}

fn part_2(input: &str) -> String {
    let mut res: u32 = 0;
    for line in input.lines() {
        let mut bag_min = create_bag_min();
        let (_game_id, subsets) = extract_game_info(line);
        dbg!(_game_id);
        for subset in subsets {
            dbg!(&subset);
            for (quantity, color) in subset {
                let current = bag_min.get(&color).unwrap();
                *bag_min.entry(color).or_insert(0) = max(*current, quantity);
            }
        }
        res += bag_min.values().product::<u32>();
        dbg!(&bag_min, &res);
    }
    res.to_string()
}

fn create_bag_min() -> HashMap<Colors, u32> {
    let mut bag_possibilities = HashMap::new();
    bag_possibilities.insert(Colors::R, 0);
    bag_possibilities.insert(Colors::G, 0);
    bag_possibilities.insert(Colors::B, 0);
    bag_possibilities
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("input1_ex.txt"); // same file
        let r = part_2(input);
        assert_eq!("2286", r);
    }
}
