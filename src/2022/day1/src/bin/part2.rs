mod common;

fn main() {
    let input = include_str!("input1.txt"); //same
    let output = part_2(input);
    dbg!(output);
}

fn part_2(input: &str) -> String {
    dbg!(input);
    let mut items_calo_per_elves = common::get_calo_per_elf(input);
    items_calo_per_elves.sort_by(|a, b| b.cmp(a));
    dbg!(&items_calo_per_elves);
    let max_calo: u32 = items_calo_per_elves.iter().take(3).sum();
    max_calo.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("input1_ex.txt"); //same
        let r = part_2(input);
        assert_eq!("45000", r);
    }
}
