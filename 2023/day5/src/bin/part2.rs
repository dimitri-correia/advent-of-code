use common::{get_maps, get_min_location};

use crate::common::Part::Part2;

mod common;

fn main() {
    let input = include_str!("./input1.txt"); //same
    let output = part_2(input);
    dbg!(output);
}

fn part_2(input: &str) -> String {
    dbg!(input);
    let input = get_maps(input, Part2);
    let res = get_min_location(input);

    res.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("./input1_ex.txt"); // same
        let r = part_2(input);
        assert_eq!("46", r);
    }
}
