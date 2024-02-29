use std::collections::HashSet;

fn part_1(input: &str, steps_remaining: i32) -> String {
    let (start_position, grid) = parse_input(input);
    dbg!(&start_position, &grid);
    let mut actual_pos = HashSet::new();
    actual_pos.insert(start_position);

    dbg!(&actual_pos);

    (0..steps_remaining).for_each(|_| {
        dbg!("a");
        let size = actual_pos.len();
        (0..size).for_each(|_| {
            actual_pos.remove(0);
        })
    });

    "".to_string()
}

fn parse_input(input: &str) -> (Position, Vec<Vec<Tile>>) {
    let mut start = None;
    let grid = input
        .lines()
        .enumerate()
        .map(|(x, l)| {
            l.chars()
                .enumerate()
                .map(|(y, c)| match c {
                    '#' => Tile::Rock,
                    '.' => Tile::GardenPlot,
                    'S' => {
                        start = Some(Position { x, y });
                        Tile::Rock
                    }
                    _ => panic!(),
                })
                .collect()
        })
        .collect();
    (start.unwrap(), grid)
}

#[derive(Debug, Eq, PartialEq, Hash)]
struct Position {
    x: usize,
    y: usize,
}

#[derive(Debug)]
enum Tile {
    Rock,
    Start,
    GardenPlot,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn actual_challenge() {
        let input = include_str!("input1.txt");
        let output = part_1(input, 64);
        dbg!(&output);
        assert_eq!("", output);
    }

    #[test]
    fn example_test() {
        let input = include_str!("input1_ex.txt");
        let r = part_1(input, 6);
        assert_eq!("16", r);
    }
}
