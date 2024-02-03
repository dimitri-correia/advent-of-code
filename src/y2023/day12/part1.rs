#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum SpringState {
    OK,
    KO,
    Unknown,
}

#[derive(Debug)]
struct Line {
    line_state: Vec<SpringState>,
    order: Vec<usize>,
}

fn parse_str_to_spring_state(c: char) -> SpringState {
    match c {
        '#' => SpringState::KO,
        '.' => SpringState::OK,
        '?' => SpringState::Unknown,
        _ => panic!(),
    }
}

fn part_1(input: &str) -> String {
    parse_input(input)
        .iter()
        .map(|line| get_nb_arrangements(line))
        .sum::<usize>()
        .to_string()
}

fn get_nb_arrangements(line: &Line) -> usize {
    generate_lines_possible(line)
        .iter()
        .filter(|possibility| check_validity(possibility, line.order.clone()))
        .count()
}

fn generate_lines_possible(line: &Line) -> Vec<Vec<SpringState>> {
    let mut result = Vec::new();
    generate_strings_helper(&line.line_state, 0, Vec::new(), &mut result);
    result
}

fn generate_strings_helper(
    line: &[SpringState],
    index: usize,
    mut current: Vec<SpringState>,
    result: &mut Vec<Vec<SpringState>>,
) {
    if index == line.len() {
        result.push(current);
    } else {
        let current_state = line[index];

        if current_state == SpringState::Unknown {
            generate_strings_helper(
                line,
                index + 1,
                {
                    let mut new_current = current.clone();
                    new_current.push(SpringState::OK);
                    new_current
                },
                result,
            );

            current.push(SpringState::KO);
            generate_strings_helper(line, index + 1, current, result);
        } else {
            current.push(current_state);
            generate_strings_helper(line, index + 1, current, result);
        }
    }
}

fn check_validity(line: &&Vec<SpringState>, expected: Vec<usize>) -> bool {
    let mut counts = Vec::new();
    let mut current_count = 0;

    for state in line.iter() {
        if state == &SpringState::KO {
            current_count += 1;
        } else if current_count > 0 {
            counts.push(current_count);
            current_count = 0;
        }
    }

    if current_count > 0 {
        counts.push(current_count);
    }

    counts == expected
}

fn parse_input(input: &str) -> Vec<Line> {
    input
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line.split_whitespace().collect();
            let line_state: Vec<SpringState> = parts[0]
                .chars()
                .map(|c| parse_str_to_spring_state(c))
                .collect();
            let order: Vec<usize> = parts[1]
                .split(',')
                .map(|s| s.parse::<usize>().unwrap())
                .collect();
            Line { line_state, order }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn actual_challenge() {
        let input = include_str!("input1.txt");
        let output = part_1(input);
        dbg!(&output);
        assert_eq!("7460", output);
    }

    #[test]
    fn example_test() {
        let input = include_str!("input1_ex.txt"); // same file
        let r = part_1(input);
        assert_eq!("21", r);
    }
}
