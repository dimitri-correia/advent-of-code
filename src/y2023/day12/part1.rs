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
    let map = parse_input(input);

    let mut r = 0;
    for line in map {
        r += get_nb_arrangements(line);
    }

    r.to_string()
}

fn get_nb_arrangements(line: Line) -> usize {
    let mut r = 0;
    for possibility in generate_lines_possible(&line) {
        if count_hashes_in_row(possibility) == line.order {
            r += 1;
        }
    }
    r
}

fn generate_lines_possible(line: &Line) -> Vec<Vec<SpringState>> {
    let mut result = Vec::new();
    generate_strings_helper(&line.line_state, 0, Vec::new(), &mut result);
    result
}

fn generate_strings_helper(
    line: &Vec<SpringState>,
    index: usize,
    mut current: Vec<SpringState>,
    result: &mut Vec<Vec<SpringState>>,
) {
    if index == line.len() {
        result.push(current);
    } else {
        let current_state = *line.get(index).unwrap();

        if current_state == SpringState::Unknown {
            let mut new_current = current.clone();
            new_current.push(SpringState::OK);
            generate_strings_helper(line, index + 1, new_current, result);

            current.push(SpringState::KO);
            generate_strings_helper(line, index + 1, current, result);
        } else {
            current.push(current_state);
            generate_strings_helper(line, index + 1, current, result);
        }
    }
}

fn count_hashes_in_row(line: Vec<SpringState>) -> Vec<usize> {
    let mut counts = Vec::new();
    let mut current_count = 0;

    for state in line {
        if state == SpringState::KO {
            current_count += 1;
        } else if current_count > 0 {
            counts.push(current_count);
            current_count = 0;
        }
    }

    if current_count > 0 {
        counts.push(current_count);
    }

    counts
}

fn parse_input(input: &str) -> Vec<Line> {
    let mut map = Vec::with_capacity(input.lines().count());
    for line in input.lines() {
        let mut line_state = Vec::with_capacity(line.len());
        let parts: Vec<&str> = line.split_whitespace().collect();
        for c in parts[0].chars() {
            let state = parse_str_to_spring_state(c);
            line_state.push(state);
        }
        let order: Vec<usize> = parts[1]
            .split(',')
            .map(|s| s.parse::<usize>().unwrap())
            .collect();
        map.push(Line { line_state, order })
    }

    map
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
