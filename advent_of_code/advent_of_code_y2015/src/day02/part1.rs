pub fn part_1_example() -> String {
    let input = include_str!("input1_ex.txt");
    part_1(input)
}

pub fn part_1_actual_challenge() -> String {
    let input = include_str!("input1.txt");
    part_1(input)
}

fn part_1(input: &str) -> String {
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
            (2 * l * w) + (2 * w * h) + (2 * h * l) + (l * w)
        })
        .sum::<u32>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn actual_challenge() {
        let output = part_1_actual_challenge();        
        assert_eq!("1598415", output);
    }

    #[test]
    fn example_test() {
        let r = part_1_example();
        assert_eq!((58 + 43).to_string(), r);
    }
}
