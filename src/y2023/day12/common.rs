#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum SpringState {
    OK,
    KO,
    Unknown,
}

#[derive(Debug)]
pub struct Line {
    pub line_state: Vec<SpringState>,
    pub groups: Vec<usize>,
}

pub fn get_nb_arrangements(line: &Line) -> usize {
    if line.groups.is_empty() {
        return if line
            .line_state
            .iter()
            .any(|state| state == &SpringState::KO)
        {
            0 // not possible
        } else {
            1
        };
    }

    // no more springs left
    if line.line_state.is_empty() {
        return 0;
    }

    let next_state = &line.line_state[0];
    let next_group = line.groups[0];
    match next_state {
        SpringState::KO => add_ko(line, next_group),
        SpringState::OK => add_ok(line),
        SpringState::Unknown => add_ko(line, next_group) + add_ok(line),
    }
}

fn add_ok(line: &Line) -> usize {
    get_nb_arrangements(&Line {
        line_state: line.line_state[1..].to_vec(),
        groups: line.groups.clone(),
    })
}

fn add_ko(line: &Line, next_group: usize) -> usize {
    if line.line_state.len() < next_group
        || line.line_state[..next_group].contains(&SpringState::OK)
    {
        return 0; // not possible
    }

    if line.line_state.len() == next_group {
        return if line.groups.len() == 1 { 1 } else { 0 };
    }

    match line.line_state[next_group] {
        SpringState::KO => 0, // next spring cannot be #
        _ => get_nb_arrangements(&Line {
            line_state: line.line_state[(next_group + 1)..].to_vec(),
            groups: line.groups[1..].to_vec(),
        }),
    }
}

pub fn parse_input(input: &str) -> Vec<Line> {
    input
        .lines()
        .map(|line| {
            let (line_state, groups) = line.split_once(' ').unwrap();
            let line_state: Vec<SpringState> = line_state
                .chars()
                .map(|c| parse_str_to_spring_state(c))
                .collect();
            let groups: Vec<usize> = groups
                .split(',')
                .map(|s| s.parse::<usize>().unwrap())
                .collect();
            Line { line_state, groups }
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
