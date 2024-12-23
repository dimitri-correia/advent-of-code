use std::cmp::min;

pub fn part_1_example() -> String {
    let input = include_str!("input1_ex.txt");
    part_1(input)
}

pub fn part_1_actual_challenge() -> String {
    let input = include_str!("input1.txt");
    part_1(input)
}

fn part_1(input: &str) -> String {
    let lines: Vec<&str> = input.lines().collect();

    let mut res = 0;

    input.lines().enumerate().for_each(|(idx, line)| {
        let mut char_index = 0;

        while char_index < line.len() {
            let c = line.chars().nth(char_index).unwrap();
            if !c.is_ascii_digit() {
                char_index += 1;
                continue;
            }
            // c is a digit
            let end = get_whole_number(line, char_index);
            if symbol_adjacent(&lines, idx, char_index, end) {
                let number = line[char_index..end].parse::<usize>().unwrap();
                res += number;
            }

            char_index = end;
        }
    });

    res.to_string()
}

fn symbol_adjacent(lines: &Vec<&str>, line_number: usize, start: usize, end: usize) -> bool {
    let line_len = lines[0].len();
    let lines_len = lines.len();

    for idx in start..end {
        let val_left = if idx > 0 { idx - 1 } else { 0 };
        let val_right = min(idx + 1, line_len - 1);
        let val_up = if line_number > 0 { line_number - 1 } else { 0 };
        let val_down = min(line_number + 1, lines_len - 1);

        let current_line = lines[line_number].as_bytes();
        let left = current_line[val_left];
        let right = current_line[val_right];
        let up = lines[val_up].as_bytes()[idx];
        let down = lines[val_down].as_bytes()[idx];
        let left_up = lines[val_up].as_bytes()[val_left];
        let left_down = lines[val_down].as_bytes()[val_left];
        let right_up = lines[val_up].as_bytes()[val_right];
        let right_down = lines[val_down].as_bytes()[val_right];

        for &c in [
            left, right, up, down, left_up, left_down, right_up, right_down,
        ]
        .iter()
        {
            if !c.is_ascii_digit() && c != b'.' {
                return true;
            }
        }
    }

    false
}

fn get_whole_number(line: &str, index: usize) -> usize {
    let mut end = index;

    while end < line.len()
        && line[end..]
            .chars()
            .next()
            .map_or(false, |c| c.is_ascii_digit())
    {
        end += 1;
    }

    end
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn actual_challenge() {
        let output = part_1_actual_challenge();        
        assert_eq!("532331", output);
    }

    #[test]
    fn example_test() {
        let input = include_str!("input1_ex.txt");

        let r = part_1(input);

        assert_eq!("4361", r);
    }
}
