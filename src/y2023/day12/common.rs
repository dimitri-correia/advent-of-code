use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
pub enum SpringState {
    OK,
    KO,
    Unknown,
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub struct Line {
    pub line_state: Vec<SpringState>,
    pub groups: Vec<usize>,
}

pub fn get_nb_arrangements(line: &Line) -> usize {
    let mut memo: HashMap<Line, usize> = HashMap::new();
    get_nb_arrangements_memo(line, &mut memo)
}

fn get_nb_arrangements_memo(line: &Line, memo: &mut HashMap<Line, usize>) -> usize {
    // check memo
    if let Some(&result) = memo.get(line) {
        return result;
    }

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
    let r = match next_state {
        SpringState::KO => add_ko(line, next_group, memo),
        SpringState::OK => add_ok(line, memo),
        SpringState::Unknown => add_ko(line, next_group, memo) + add_ok(line, memo),
    };

    // memoize the result
    memo.insert(line.clone(), r);

    r
}

fn add_ok(line: &Line, memo: &mut HashMap<Line, usize>) -> usize {
    get_nb_arrangements_memo(
        &Line {
            line_state: line.line_state[1..].to_vec(),
            groups: line.groups.clone(),
        },
        memo,
    )
}

fn add_ko(line: &Line, next_group: usize, memo: &mut HashMap<Line, usize>) -> usize {
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
        _ => get_nb_arrangements_memo(
            &Line {
                line_state: line.line_state[(next_group + 1)..].to_vec(),
                groups: line.groups[1..].to_vec(),
            },
            memo,
        ),
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
