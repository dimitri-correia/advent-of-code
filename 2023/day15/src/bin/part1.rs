fn main() {
    let input = include_str!("./input1.txt");
    let output = part_1(input);
    dbg!(output);
}

fn part_1(input: &str) -> String {
    input
        .split(',')
        .map(|hash| {
            hash.chars()
                .fold(0, |acc, next_char| (acc + (next_char as usize)) * 17 % 256)
        })
        .sum::<usize>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("./input1_ex.txt");
        let r = part_1(input);
        assert_eq!("1320", r);
    }
}
