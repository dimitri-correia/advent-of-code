use crate::Shape::RoundedRock;

fn main() {
    let input = include_str!("./input1.txt");
    let output = part_1(input);
    dbg!(output);
}

fn part_1(input: &str) -> String {
    parse_input(input)
        .into_iter()
        .map(get_val_col)
        .sum::<usize>()
        .to_string()
}

fn get_val_col(col: Vec<Shape>) -> usize {
    let mut r = 0;
    let mut curr_rounded_tot = 0;
    let len = col.len();
    let mut last_non_cube_rock = len;
    for (idx, shape) in col.iter().enumerate() {
        if shape == &Shape::CubeRock {
            for i in last_non_cube_rock - curr_rounded_tot + 1..=last_non_cube_rock {
                r += i;
            }
            curr_rounded_tot = 0;
            last_non_cube_rock = len - idx - 1;
        } else if shape == &RoundedRock {
            curr_rounded_tot += 1;
        }
    }
    for i in last_non_cube_rock - curr_rounded_tot + 1..=last_non_cube_rock {
        r += i;
    }
    dbg!(&r);
    r
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Shape {
    RoundedRock,
    CubeRock,
    Empty,
}

fn parse_char(c: char) -> Shape {
    match c {
        'O' => Shape::RoundedRock,
        '.' => Shape::Empty,
        '#' => Shape::CubeRock,
        _ => panic!(),
    }
}

fn parse_input(input: &str) -> Vec<Vec<Shape>> {
    let mut grid = Vec::new();

    for line in input.lines() {
        let row: Vec<Shape> = line.chars().map(|c| parse_char(c)).collect();
        grid.push(row);
    }

    let rows = grid.len();
    let cols = grid[0].len();

    (0..cols)
        .map(|col| (0..rows).map(|row| grid[row][col]).collect())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("./input1_ex.txt");
        let r = part_1(input);
        assert_eq!("136", r);
    }
}
