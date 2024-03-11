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

pub fn parse_str_to_spring_state(c: char) -> SpringState {
    match c {
        '#' => SpringState::KO,
        '.' => SpringState::OK,
        '?' => SpringState::Unknown,
        _ => unreachable!(),
    }
}
