use std::collections::HashMap;

fn main() {
    let input = include_str!("./input1.txt");
    let output = part_1(input);
    dbg!(output);
}

fn part_1(input: &str) -> String {
    let (pattern, map): (&str, HashMap<String, MapLine>) = read_input(input);

    dbg!(&map);
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

    dbg!(pattern);

    //remove empty line
    lines.next();

    let mut map = HashMap::new();

    for line in lines {
        let (actual_pos, map_line) = parse_line(line);
        map.insert(actual_pos, map_line);
    }
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
    fn it_works_1() {
        let input = include_str!("./input1_ex.txt");
        let r = part_1(input);
        assert_eq!("2", r);
    }

    #[test]
    fn it_works_2() {
        let input = include_str!("./input1_ex2.txt");
        let r = part_1(input);
        assert_eq!("6", r);
    }
}
