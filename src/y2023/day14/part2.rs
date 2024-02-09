use crate::y2023::day14::common::{
    change_oder_col_row, get_val_col, parse_input_col_row, Grid, Order, Shape,
};

fn part_2(input: &str) -> String {
    const NUMBER_CYCLE: usize = 1_000_000_000;

    // as first element of cycle is north, we need to have the first grid in row_col format
    let grid = parse_input_col_row(input);

    // also in row_col format
    let final_grid = get_final_grid(grid, NUMBER_CYCLE);

    final_grid
        .grid
        .into_iter()
        .map(get_val_col) // todo
        .sum::<usize>()
        .to_string()
}

fn get_final_grid(grid: Grid, number_cycle: usize) -> Grid {
    assert_eq!(grid.order, Order::ColRow);
    let final_grid = (0..number_cycle).into_iter().fold(grid, |tmp_grid, _| {
        do_east(do_south(do_west(do_north(tmp_grid))))
    });
    assert_eq!(final_grid.order, Order::ColRow);
    final_grid
}

fn do_east(grid: Grid) -> Grid {
    assert_eq!(grid.order, Order::RowCol);
    let new_grid = change_oder_col_row(grid);

    Grid {
        grid: new_grid
            .grid
            .iter()
            .map(|line| move_o_right(line))
            .collect(),
        order: new_grid.order,
    }
}

fn do_south(grid: Grid) -> Grid {
    assert_eq!(grid.order, Order::ColRow);
    let new_grid = change_oder_col_row(grid);

    Grid {
        grid: new_grid
            .grid
            .iter()
            .map(|line| move_o_right(line))
            .collect(),
        order: new_grid.order,
    }
}

fn do_west(grid: Grid) -> Grid {
    assert_eq!(grid.order, Order::RowCol);
    let new_grid = change_oder_col_row(grid);

    Grid {
        grid: new_grid.grid.iter().map(|line| move_o_left(line)).collect(),
        order: new_grid.order,
    }
}

fn do_north(grid: Grid) -> Grid {
    assert_eq!(grid.order, Order::ColRow);
    let new_grid = change_oder_col_row(grid);

    Grid {
        grid: new_grid.grid.iter().map(|line| move_o_left(line)).collect(),
        order: new_grid.order,
    }
}

fn move_o_right(v: &Vec<Shape>) -> Vec<Shape> {
    let mut new_v: Vec<Shape> = vec![];
    let mut rounded = 0;
    let mut empty = 0;
    for shape in v.iter().rev() {
        match shape {
            Shape::CubeRock => {
                (0..rounded)
                    .into_iter()
                    .for_each(|_| new_v.push(Shape::RoundedRock));
                (0..empty)
                    .into_iter()
                    .for_each(|_| new_v.push(Shape::Empty));
                new_v.push(Shape::CubeRock);
                rounded = 0;
                empty = 0;
            }
            Shape::RoundedRock => rounded += 1,
            Shape::Empty => empty += 1,
        }
    }
    (0..rounded)
        .into_iter()
        .for_each(|_| new_v.push(Shape::RoundedRock));
    (0..empty)
        .into_iter()
        .for_each(|_| new_v.push(Shape::Empty));
    new_v.reverse();
    new_v
}

fn move_o_left(v: &Vec<Shape>) -> Vec<Shape> {
    let mut new_v: Vec<Shape> = vec![];
    let mut rounded = 0;
    let mut empty = 0;
    for shape in v {
        match shape {
            Shape::CubeRock => {
                (0..rounded)
                    .into_iter()
                    .for_each(|_| new_v.push(Shape::RoundedRock));
                (0..empty)
                    .into_iter()
                    .for_each(|_| new_v.push(Shape::Empty));
                new_v.push(Shape::CubeRock);
                rounded = 0;
                empty = 0;
            }
            Shape::RoundedRock => rounded += 1,
            Shape::Empty => empty += 1,
        }
    }
    (0..rounded)
        .into_iter()
        .for_each(|_| new_v.push(Shape::RoundedRock));
    (0..empty)
        .into_iter()
        .for_each(|_| new_v.push(Shape::Empty));
    new_v
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::y2023::day14::common::{parse_char, parse_input_row_col};
    use itertools::Itertools;

    // #[test]
    // fn actual_challenge() {
    //     let input = include_str!("input1.txt");
    //     let output = part_2(input);
    //     dbg!(&output);
    //     assert_eq!("val", output);
    // }
    //
    #[test]
    fn example_test() {
        let input = include_str!("input1_ex.txt"); // same file
        let r = part_2(input);
        assert_eq!("64", r);
    }

    #[test]
    fn test_do_north() {
        let input = include_str!("input1_ex.txt");
        let before = parse_input_col_row(input);
        let expected = parse_input_row_col(
            "OOOO.#.O..
OO..#....#
OO..O##..O
O..#.OO...
........#.
..#....#.#
..O..#.O.O
..O.......
#....###..
#....#....",
        );
        let computed = do_north(before);

        helper_print(&computed);
        helper_print(&expected);

        assert_eq!(expected, computed);
    }

    #[test]
    fn test_move_o_right() {
        let before = "..O#O..O..#..O.."
            .chars()
            .map(parse_char)
            .collect::<Vec<Shape>>();
        let expected = "..O#....OO#....O"
            .chars()
            .map(parse_char)
            .collect::<Vec<Shape>>();
        let computed = move_o_right(&before);

        assert_eq!(expected, computed);
    }

    #[test]
    fn test_move_o_left() {
        let before = "..O.#O..O..#..O.."
            .chars()
            .map(parse_char)
            .collect::<Vec<Shape>>();
        let expected = "O...#OO....#O...."
            .chars()
            .map(parse_char)
            .collect::<Vec<Shape>>();
        let computed = move_o_left(&before);

        assert_eq!(expected, computed);
    }

    #[test]
    fn test_3_cycles() {
        let input = include_str!("input1_ex.txt"); // same file
        let grid = parse_input_col_row(input);
        let expected = [
            ".....#....
....#...O#
...OO##...
.OO#......
.....OOO#.
.O#...O#.#
....O#....
......OOOO
#...O###..
#..OO#....",
            ".....#....
....#...O#
.....##...
..O#......
.....OOO#.
.O#...O#.#
....O#...O
.......OOO
#..OO###..
#.OOO#...O",
            ".....#....
....#...O#
.....##...
..O#......
.....OOO#.
.O#...O#.#
....O#...O
.......OOO
#...O###.O
#.OOO#...O",
        ];
        let mut new_grid = grid.clone();
        for ex in expected {
            new_grid = get_final_grid(new_grid, 1);
            let res = parse_input_col_row(ex);
            helper_print(&res);
            assert_eq!(res.order, new_grid.order);
            assert_eq!(res.grid, new_grid.grid);
            dbg!("one cycle ok");
        }
    }

    fn helper_print(v: &Grid) -> String {
        fn inverse_parse(s: &Shape) -> char {
            match s {
                Shape::RoundedRock => 'O',
                Shape::Empty => '.',
                Shape::CubeRock => '#',
            }
        }
        let g = if v.order == Order::RowCol {
            change_oder_col_row(v.clone())
        } else {
            v.clone()
        };
        let p = g
            .grid
            .iter()
            .map(|row| row.iter().map(|s| inverse_parse(s)).collect::<String>())
            .join("\n");

        dbg!(p)
    }
}
