use std::collections::HashSet;

pub fn part_2_example() -> String {
    let input = include_str!("input1_ex.txt");
    part_2(input)
}

pub fn part_2_actual_challenge() -> String {
    let input = include_str!("input1.txt");
    part_2(input)
}

fn part_2(input: &str) -> String {
    let mut visited = HashSet::new();
    visited.insert((0, 0));
    input.char_indices().fold(((0, 0), (0, 0)), |last, c| {
        let new;
        if c.0 % 2 == 0 {
            new = get_next(c.1, last.0);
            visited.insert(new);
            (new, last.1)
        } else {
            new = get_next(c.1, last.1);
            visited.insert(new);
            (last.0, new)
        }
    });
    visited.len().to_string()
}

fn get_next(c: char, last: (i32, i32)) -> (i32, i32) {
    match c {
        '>' => (last.0 + 1, last.1),
        '<' => (last.0 - 1, last.1),
        '^' => (last.0, last.1 + 1),
        'v' => (last.0, last.1 - 1),
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn actual_challenge() {
        let output = part_2_actual_challenge();        assert_eq!("2639", output);
    }
    #[test]
    fn example_test() {
        let r = part_2_example();
        assert_eq!("11", r);
    }
}
