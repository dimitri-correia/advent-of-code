mod common;

fn main() {
    let input = include_str!("./input1.txt");
    let output = part_1(input);
    dbg!(output);
}

fn part_1(input: &str) -> String {
    dbg!(input);
    let items_calo_per_elves = common::get_calo_per_elf(input);
    dbg!(&items_calo_per_elves);
    let max_calo = items_calo_per_elves.iter().max().unwrap();
    max_calo.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("./input1_ex.txt");
        let r = part_1(input);
        assert_eq!("24000", r);
    }
}
