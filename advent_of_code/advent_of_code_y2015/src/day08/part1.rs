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
        .map(|l| l.len() - get_char_string_data(&l[1..l.len() - 1]))
        .sum::<usize>()
        .to_string()
}

fn get_char_string_data(l: &str) -> usize {
    let mut count = 0;
    let mut iter = l.chars();
    while let Some(c) = iter.next() {
        count += 1;
        if c == '\\' {
            let next = iter.next().unwrap();
            if next == 'x' {
                iter.next();
                iter.next();
            }
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn actual_challenge() {
        let output = part_1_actual_challenge();        
        assert_eq!("1333", output);
    }

    #[test]
    fn example_test() {
        let r = part_1_example();
        assert_eq!("12", r);
    }
}
