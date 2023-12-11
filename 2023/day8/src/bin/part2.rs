use std::collections::HashMap;
use std::str::Chars;

fn main() {
    let input = include_str!("./input1.txt"); //same
    let output = part_2(input);
    dbg!(output);
}

fn part_2(input: &str) -> String {
    let (pattern, map, mut starting_points): (Chars, HashMap<String, MapLine>, Vec<String>) =
        read_input(input);

    let mut pattern = pattern
        .map(|c| match c {
            'L' => Direction::Left,
            _ => Direction::Right,
        })
        .cycle();

    let mut i = 0;
    while !starting_points.iter().all(|p| map.get(p).unwrap().end) {
        match pattern.next().unwrap() {
            Direction::Left => {
                starting_points = starting_points
                    .iter()
                    .flat_map(|p| map.get(p).map(|line| line.left.clone()))
                    .collect();
            }
            _ => {
                starting_points = starting_points
                    .iter()
                    .flat_map(|p| map.get(p).map(|line| line.right.clone()))
                    .collect();
            }
        }

        i += 1;
    }

    i.to_string()
}

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

fn read_input(input: &str) -> (Chars, HashMap<String, MapLine>, Vec<String>) {
    let mut lines = input.lines();

    let pattern = lines.next().unwrap().chars();

    //remove empty line
    lines.next();

    let mut map = HashMap::new();
    let mut starting_points = vec![];

    for line in lines {
        let (actual_pos, map_line, start) = parse_line(line);
        map.insert(actual_pos.clone(), map_line);
        if start {
            starting_points.push(actual_pos);
        }
    }
    (pattern, map, starting_points)
}

#[derive(Debug)]
struct MapLine {
    left: String,
    right: String,
    end: bool,
}

fn parse_line(input: &str) -> (String, MapLine, bool) {
    let cleaned_input: String = input
        .chars()
        .filter(|&c| c.is_alphanumeric() || c == ',' || c == '=')
        .collect();
    let parts: Vec<&str> = cleaned_input.split('=').collect();
    let actual_pos = parts[0].trim().to_string();
    let parts: Vec<&str> = parts[1].split(',').collect();
    let left = parts[0].trim().to_string();
    let right = parts[1].trim().to_string();
    let start = actual_pos.ends_with('A');
    let end = actual_pos.ends_with('Z');

    (actual_pos, MapLine { left, right, end }, start)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("./input2_ex.txt");
        let r = part_2(input);
        assert_eq!("6", r);
    }
}
