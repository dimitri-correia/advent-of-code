pub fn part_2_example() -> String {
    let input = include_str!("input1_ex.txt");
    part_2(input)
}

pub fn part_2_actual_challenge() -> String {
    let input = include_str!("input1.txt");
    part_2(input)
}

fn part_2(input: &str) -> String {
    input
        .lines()
        .map(|line| {
            let mut dims = line
                .split('x')
                .map(|x| x.parse::<u32>().unwrap())
                .collect::<Vec<u32>>();
            dims.sort();
            let l = dims[0];
            let w = dims[1];
            let h = dims[2];
            (2 * l) + (2 * w) + (l * w * h)
        })
        .sum::<u32>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn actual_challenge() {
        let output = part_2_actual_challenge();        assert_eq!("3812909", output);
    }
    #[test]
    fn example_test() {
        let r = part_2_example();
        assert_eq!((34 + 14).to_string(), r);
    }
}
