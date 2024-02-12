#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
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

pub fn parse_input_row_col(input: &str) -> Grid {
    let mut grid = parse_input_col_row(input);

    change_order_col_row(&mut grid);

    grid
}

pub fn parse_input_col_row(input: &str) -> Grid {
    Grid {
        grid: input
            .lines()
            .map(|l| l.chars().map(|c| parse_char(c)).collect())
            .collect(),
        order: Order::ColRow,
    }
}

#[derive(Eq, PartialEq, Clone, Debug, Hash)]
pub struct Grid {
    pub grid: Vec<Vec<Shape>>,
    pub order: Order,
}

#[derive(Eq, PartialEq, Copy, Clone, Debug, Hash)]
pub enum Order {
    RowCol,
    ColRow,
}

pub fn change_order_col_row(old: &mut Grid) {
    let rows = old.grid.len();
    let cols = old.grid[0].len();

    let new_grid = (0..cols)
        .map(|col| (0..rows).map(|row| old.grid[row][col]).collect())
        .collect();

    old.grid = new_grid;
    old.order = if old.order == Order::ColRow {
        Order::RowCol
    } else {
        Order::ColRow
    };
}
