use std::cmp::min;
use std::collections::HashSet;

pub fn part_2_example() -> String {
    let input = include_str!("input1_ex.txt");
    part_2(input)
}

pub fn part_2_actual_challenge() -> String {
    let input = include_str!("input1.txt");
    part_2(input)
}

fn part_2(input: &str) -> String {
    let lines: Vec<&str> = input.lines().collect();

    let mut res = 0;

    for (line_number, line) in lines.iter().enumerate() {
        for (char_index, c) in line.char_indices() {
            if c != '*' {
                continue;
            }
            let gear_ratio = gear_ratio(&lines, line_number, char_index);
            res += gear_ratio;
        }
    }

    res.to_string()
}

fn gear_ratio(lines: &Vec<&str>, line_number: usize, idx: usize) -> i32 {
    let line_len = lines[0].len();
    let lines_len = lines.len();

    let val_left = if idx > 0 { idx - 1 } else { 0 };
    let val_right = min(idx + 1, line_len - 1);
    let val_up = if line_number > 0 { line_number - 1 } else { 0 };
    let val_down = min(line_number + 1, lines_len - 1);

    let mut gear_values: HashSet<(usize, (usize, usize))> = HashSet::new();
    for &(row, col) in [
        (val_up, val_left),
        (val_up, val_right),
        (val_up, idx),
        (line_number, val_left),
        (line_number, val_right),
        (val_down, idx),
        (val_down, val_left),
        (val_down, val_right),
    ]
    .iter()
    {
        if lines[row].as_bytes()[col].is_ascii_digit() {
            gear_values.insert((row, get_whole_number(lines[row], col)));
        }
    }

    if gear_values.len() == 2 {
        return gear_values
            .iter()
            .map(|(row, (start, end))| lines[*row][*start..*end].parse::<i32>().unwrap())
            .product::<i32>();
    }
    0
}

fn get_whole_number(line: &str, index: usize) -> (usize, usize) {
    let mut end = index;

    while end < line.len()
        && line[end..]
            .chars()
            .next()
            .map_or(false, |c| c.is_ascii_digit())
    {
        end += 1;
    }
    let mut start = index;

    while start > 0
        && line[start - 1..start]
            .chars()
            .next()
            .map_or(false, |c| c.is_ascii_digit())
    {
        start -= 1;
    }

    (start, end)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn actual_challenge() {
        let output = part_2_actual_challenge();        assert_eq!("82301120", output);
    }

    #[test]
    fn example_test() {
        let r = part_2_example();
        assert_eq!("467835", r);
    }
}
