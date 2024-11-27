pub fn part_2_example() -> String {
    let input = "qjhvhtzxzqqjkmpb
xxyxx
uurcxstgmygtbstg
ieodomkazucvgmuy
";
    part_2(input)
}

pub fn part_2_actual_challenge() -> String {
    let input = include_str!("input1.txt");
    part_2(input)
}

fn part_2(input: &str) -> String {
    input
        .lines()
        .filter(|l| l.chars().zip(l.chars().skip(2)).any(|(a, b)| a == b))
        .inspect(|f| println!("{}", f))
        .filter(|l| {
            l.char_indices().zip(l.chars().skip(1)).any(|((i, a), b)| {
                l.chars()
                    .skip(i + 2)
                    .zip(l.chars().skip(i + 3))
                    .any(|(c, d)| a == c && b == d)
            })
        })
        .inspect(|f| println!("{}", f))
        .count()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn actual_challenge() {
        let output = part_2_actual_challenge();        assert_eq!("69", output);
    }
    #[test]
    fn example_test() {
        let r = part_2_example();
        assert_eq!("2", r);
    }
}
