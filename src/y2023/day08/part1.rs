use std::collections::HashMap;

fn part_1(input: &str) -> String {
    let (pattern, map): (&str, HashMap<String, MapLine>) = read_input(input);

    let mut pos = String::from("AAA");
    let mut i = 0;
    while pos != "ZZZ" {
        let next = pattern.chars().nth(i % pattern.len()).unwrap();
        if next == 'L' {
            pos = map.get(&pos).unwrap().left.clone();
        } else {
            pos = map.get(&pos).unwrap().right.clone();
        }
        i += 1;
    }

    i.to_string()
}

fn read_input(input: &str) -> (&str, HashMap<String, MapLine>) {
    let mut lines = input.lines();

    let pattern = lines.next().unwrap();

    // remove empty line
    lines.next();

    let map: HashMap<String, MapLine> = lines.map(|line| parse_line(line)).collect();

    (pattern, map)
}

#[derive(Debug)]
struct MapLine {
    left: String,
    right: String,
}

fn parse_line(input: &str) -> (String, MapLine) {
    let cleaned_input: String = input
        .chars()
        .filter(|&c| c.is_alphanumeric() || c == ',' || c == '=')
        .collect();

    let parts: Vec<&str> = cleaned_input.split('=').collect();
    let actual_pos = parts[0].trim().to_string();

    let parts: Vec<&str> = parts[1].split(',').collect();
    let left = parts[0].trim().to_string();
    let right = parts[1].trim().to_string();

    (actual_pos, MapLine { left, right })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn actual_challenge() {
        let input = include_str!("input1.txt");
        let output = part_1(input);
        dbg!(&output);
        assert_eq!("15871", output);
    }

    #[test]
    fn example_test_a() {
        let input = include_str!("input1_ex.txt");
        let r = part_1(input);
        assert_eq!("2", r);
    }

    #[test]
    fn example_test_b() {
        let input = include_str!("input1_ex2.txt");
        let r = part_1(input);
        assert_eq!("6", r);
    }
}
