use common::{get_maps, get_min_location};

use crate::common::Part::Part1;

mod common;

fn main() {
    let input = include_str!("./input1.txt");
    let output = part_1(input);
    dbg!(output);
}

fn part_1(input: &str) -> String {
    dbg!(input);
    let input = get_maps(input, Part1);

    let res = get_min_location(input);

    res.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("./input1_ex.txt");
        let r = part_1(input);
        assert_eq!("35", r);
    }
}
