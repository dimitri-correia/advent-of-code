mod common;
fn main() {
    let input = include_str!("input1.txt"); //same
    let output = part_2(input, 1_000_000);
    dbg!(output);
}

fn part_2(input: &str, i: usize) -> String {
    let distances = common::calculate_distances(input, i);

    let sum: usize = distances.iter().map(|d| d.dist).sum();

    sum.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_10() {
        let input = include_str!("input1_ex.txt"); // same
        let r = part_2(input, 10);
        assert_eq!("1030", r);
    }

    #[test]
    fn it_works_100() {
        let input = include_str!("input1_ex.txt"); // same
        let r = part_2(input, 100);
        assert_eq!("8410", r);
    }
}
