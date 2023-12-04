fn main() {
    let input = include_str!("./input1.txt"); // didnt changed
    let output = part_2(input);
    dbg!(output);
}

fn part_2(input: &str) -> String {
    let mut res = 0;

    input
        .split("\n")
        .filter(|line| !line.is_empty())
        .for_each(|line| {
            dbg!(line);
            for i in 0..line.len() {
                let r = match &line[i..] {
                    s if s.starts_with("one") => Some(1),
                    s if s.starts_with("two") => Some(2),
                    s if s.starts_with("three") => Some(3),
                    s if s.starts_with("four") => Some(4),
                    s if s.starts_with("five") => Some(5),
                    s if s.starts_with("six") => Some(6),
                    s if s.starts_with("seven") => Some(7),
                    s if s.starts_with("eight") => Some(8),
                    s if s.starts_with("nine") => Some(9),
                    _ => None,
                };

                if let Some(value) = r {
                    res += value * 10;
                    break;
                }

                if line.chars().nth(i).unwrap().is_ascii_digit() {
                    res += 10 * line.chars().nth(i).unwrap().to_digit(10).unwrap();
                    break;
                }
            }

            for i in (0..line.len()).rev() {
                let reduced_line = &line[i..];
                let r = match reduced_line {
                    s if s.starts_with("one") => Some(1),
                    s if s.starts_with("two") => Some(2),
                    s if s.starts_with("three") => Some(3),
                    s if s.starts_with("four") => Some(4),
                    s if s.starts_with("five") => Some(5),
                    s if s.starts_with("six") => Some(6),
                    s if s.starts_with("seven") => Some(7),
                    s if s.starts_with("eight") => Some(8),
                    s if s.starts_with("nine") => Some(9),
                    _ => None,
                };

                if let Some(value) = r {
                    res += value;
                    break;
                }

                if line.chars().nth(i).unwrap().is_ascii_digit() {
                    res += line.chars().nth(i).unwrap().to_digit(10).unwrap();
                    break;
                }
            }
        });

    res.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("./input2_ex.txt");
        let r = part_2(input);
        assert_eq!("281", r);
    }
}
