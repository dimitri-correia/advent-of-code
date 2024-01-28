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
    let mut result = vec![vec![]];

    for state in &line.line_state {
        let mut next_result = Vec::new();

        for current in &result {
            match state {
                SpringState::Unknown => {
                    let mut option1 = current.clone();
                    option1.push(SpringState::OK);
                    next_result.push(option1);

                    let mut option2 = current.clone();
                    option2.push(SpringState::KO);
                    next_result.push(option2);
                }
                _ => {
                    let mut new_current = current.clone();
                    new_current.push(*state);
                    next_result.push(new_current);
                }
            }
        }
        result = next_result;
    }

    result.len()
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
