#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Shape {
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

pub fn parse_input_row_col(input: &str) -> Vec<Vec<Shape>> {
    let col_row: Vec<Vec<Shape>> = parse_input_col_row(input);

    change_oder_col_row(col_row)
}

pub fn parse_input_col_row(input: &str) -> Vec<Vec<Shape>> {
    input
        .lines()
        .map(|l| l.chars().map(|c| parse_char(c)).collect())
        .collect()
}

pub fn change_oder_col_row(old: Vec<Vec<Shape>>) -> Vec<Vec<Shape>> {
    let rows = old.len();
    let cols = old[0].len();

    (0..cols)
        .map(|col| (0..rows).map(|row| old[row][col]).collect())
        .collect()
}

pub fn get_val_col(col: Vec<Shape>) -> usize {
    let len = col.len();

    let mut r = 0;
    let mut curr_rounded_tot = 0;
    let mut last_non_cube_rock = len;

    for (idx, shape) in col.iter().enumerate() {
        if shape == &Shape::CubeRock {
            r += (last_non_cube_rock - curr_rounded_tot + 1..=last_non_cube_rock).sum::<usize>();
            curr_rounded_tot = 0;
            last_non_cube_rock = len - idx - 1;
        } else if shape == &Shape::RoundedRock {
            curr_rounded_tot += 1;
        }
    }

    r + (last_non_cube_rock - curr_rounded_tot + 1..=last_non_cube_rock).sum::<usize>()
}
