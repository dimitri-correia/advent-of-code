#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum SpringState {
    OK,
    KO,
    Unknown,
}

#[derive(Debug)]
pub struct Line {
    pub line_state: Vec<SpringState>,
    pub order: Vec<usize>,
}

pub fn parse_input(input: &str) -> Vec<Line> {
    input
        .lines()
        .map(|line| {
            let (line_state, order) = line.split_once(' ').unwrap();
            let line_state: Vec<SpringState> = line_state
                .chars()
                .map(|c| parse_str_to_spring_state(c))
                .collect();
            let order: Vec<usize> = order
                .split(',')
                .map(|s| s.parse::<usize>().unwrap())
                .collect();
            Line { line_state, order }
        })
        .collect()
}

fn parse_str_to_spring_state(c: char) -> SpringState {
    match c {
        '#' => SpringState::KO,
        '.' => SpringState::OK,
        '?' => SpringState::Unknown,
        _ => unreachable!(),
    }
}
