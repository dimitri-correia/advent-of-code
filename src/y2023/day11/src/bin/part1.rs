mod common;

fn main() {
    let input = include_str!("input1.txt");
    let output = part_1(input);
    dbg!(output);
}

fn part_1(input: &str) -> String {
    let distances = common::calculate_distances(input, 1);

    let sum: usize = distances.iter().map(|d| d.dist).sum();

    sum.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("input1_ex.txt");
        let r = part_1(input);
        assert_eq!("374", r);
    }
}
