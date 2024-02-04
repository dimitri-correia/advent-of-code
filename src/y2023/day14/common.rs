#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Shape {
    RoundedRock,
    CubeRock,
    Empty,
}

pub fn parse_char(c: char) -> Shape {
    match c {
        'O' => Shape::RoundedRock,
        '.' => Shape::Empty,
        '#' => Shape::CubeRock,
        _ => panic!(),
    }
}

pub fn parse_input(input: &str) -> Vec<Vec<Shape>> {
    let grid: Vec<Vec<Shape>> = input
        .lines()
        .map(|l| l.chars().map(|c| parse_char(c)).collect())
        .collect();

    let rows = grid.len();
    let cols = grid[0].len();

    (0..cols)
        .map(|col| (0..rows).map(|row| grid[row][col]).collect())
        .collect()
}
