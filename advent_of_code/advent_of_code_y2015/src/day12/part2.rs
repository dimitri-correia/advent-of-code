use core::str;

pub fn part_2_example() -> String {
    let input = include_str!("input1_ex.txt");
    part_2(input)
}

pub fn part_2_actual_challenge() -> String {
    let input = include_str!("input1.txt");
    part_2(input)
}

fn part_2(input: &str) -> String {
    let mut chars = input.chars();
    let mut sum = 0;
    let mut curr = 0;
    let mut sign = 1;
    loop {
        let c = chars.next();
        if c.is_none() {
            break;
        }
        let c = c.unwrap();
        match c {
            '-' | '0'..='9' => {
                let (num, c) = parse_number(&mut chars, c);
                match c {
                    ']' => {
                        sum += num * sign;
                        break;
                    }
                    '}' => {
                        sum += num * sign;
                        break;
                    }
                    _ => {
                        curr = num;
                        continue;
                    }
                }
            }
            '[' => {
                sum += part_2(str::from_utf8(chars.as_str().as_bytes()).unwrap())
                    .parse::<i32>()
                    .unwrap();
                curr = 0;
                sign = 1;
                continue;
            }
            ']' => {
                sum += curr * sign;
                break;
            }
            _ => {
                sum += curr * sign;
                curr = 0;
                sign = 1;
                continue;
            }
        }
    }
    sum.to_string()
}

fn parse_number(chars: &mut str::Chars, c: char) -> (i32, char) {
    let mut num = if c == '-' {
        -1
    } else {
        c.to_digit(10).unwrap() as i32
    };
    loop {
        let c = chars.next();
        if c.is_none() {
            break;
        }
        let c = c.unwrap();
        match c {
            '0'..='9' => {
                num = num * 10 + c.to_digit(10).unwrap() as i32;
            }
            _ => {
                break;
            }
        }
    }
    (num, c)
}

struct JsonParser {
    sum: Box<JsonParser>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn actual_challenge() {
        let output = part_2_actual_challenge();        assert_eq!("", output);
    }
    #[test]
    fn example_test() {
        let r = part_2_example();
        assert_eq!("", r);
    }
}
