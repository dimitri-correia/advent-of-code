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

    let rows = col_row.len();
    let cols = col_row[0].len();

    (0..cols)
        .map(|col| (0..rows).map(|row| col_row[row][col]).collect())
        .collect()
}

pub fn parse_input_col_row(input: &str) -> Vec<Vec<Shape>> {
    input
        .lines()
        .map(|l| l.chars().map(|c| parse_char(c)).collect())
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
