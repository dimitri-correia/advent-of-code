use itertools::Itertools;
use std::slice::Iter;

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
    pub count_unknown: usize,
}

pub fn count_unknown(line: &Vec<SpringState>) -> usize {
    line.iter().filter(|s| s == &&SpringState::Unknown).count()
}

pub fn parse_str_to_spring_state(c: char) -> SpringState {
    match c {
        '#' => SpringState::KO,
        '.' => SpringState::OK,
        '?' => SpringState::Unknown,
        _ => unreachable!(),
    }
}

pub fn get_nb_arrangements(line: &Line) -> usize {
    itertools::repeat_n([SpringState::OK, SpringState::KO], line.count_unknown)
        .multi_cartesian_product()
        .filter(|o| check_validity(o.iter(), line))
        .count()
}

fn check_validity(mut arrangement: Iter<SpringState>, line: &Line) -> bool {
    let completed_line: Vec<&SpringState> = line
        .line_state
        .iter()
        .map(|c| match c {
            SpringState::Unknown => arrangement.next().unwrap(),
            value => value,
        })
        .collect();
    check_validityy(completed_line, &line.order)
}

fn check_validityy(line: Vec<&SpringState>, expected: &Vec<usize>) -> bool {
    let mut counts = Vec::new();
    let mut current_count = 0;

    for state in line.iter() {
        if state == &&SpringState::KO {
            current_count += 1;
        } else if current_count > 0 {
            counts.push(current_count);
            current_count = 0;
        }
    }

    if current_count > 0 {
        counts.push(current_count);
    }

    &counts == expected
}
