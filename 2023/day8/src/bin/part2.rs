use std::collections::HashMap;
use std::str::Chars;

fn main() {
    let input = include_str!("./input1.txt"); //same
    let output = part_2(input);
    dbg!(output);
}

pub fn part_2(input: &str) -> String {
    let (pattern, map, starting_points): (Chars, HashMap<String, MapLine>, Vec<String>) =
        read_input(input);

    let results = starting_points
        .iter()
        .map(|node| {
            let mut current_node = node.clone();
            pattern
                .clone()
                .map(|c| match c {
                    'L' => Direction::Left,
                    _ => Direction::Right,
                })
                .cycle()
                .enumerate()
                .find_map(|(index, instruction)| {
                    let next_node = map.get(&current_node).unwrap();
                    if next_node.end {
                        return Some(index);
                    }
                    current_node = match instruction {
                        Direction::Left => next_node.left.clone(),
                        Direction::Right => next_node.right.clone(),
                    };
                    None
                })
                .unwrap()
        })
        .collect::<Vec<usize>>();

    dbg!(&results);

    let min_cycle = calculate_least_common_multiple(&results);

    min_cycle.to_string()
}

pub fn calculate_least_common_multiple(nums: &[usize]) -> usize {
    match nums.len() {
        0 => 1,
        1 => nums[0],
        _ => {
            let mut result = nums[0];
            for &num in &nums[1..] {
                result = result * num / calculate_greatest_common_divisor(result, num);
            }
            result
        }
    }
}

fn calculate_greatest_common_divisor(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        calculate_greatest_common_divisor(b, a % b)
    }
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
