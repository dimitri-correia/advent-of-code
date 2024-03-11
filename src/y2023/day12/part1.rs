use crate::y2023::day12::common::{parse_input, Line, SpringState};

fn part_1(input: &str) -> String {
    parse_input(input)
        .iter()
        .map(|line| get_nb_arrangements(line))
        .sum::<usize>()
        .to_string()
}

fn get_nb_arrangements(line: &Line) -> usize {
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
        let input = include_str!("input1_ex.txt");
        let r = part_1(input);
        assert_eq!("21", r);
    }
}
