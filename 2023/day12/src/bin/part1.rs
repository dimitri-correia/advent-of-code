fn main() {
    let input = include_str!("./input1.txt");
    let output = part_1(input);
    dbg!(output);
}

#[derive(Debug)]
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
    dbg!(&input);

    let map = parse_input(input);
    dbg!(&map);

    let mut r = 0;
    for line in map {
        r += get_nb_arrangements(line);
    }

    r.to_string()
}

fn get_nb_arrangements(line: Line) -> usize {
    0
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
    fn it_works() {
        let input = include_str!("./input1_ex.txt");
        let r = part_1(input);
        assert_eq!("21", r);
    }
}
