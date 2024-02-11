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
    (0..number_cycle).into_iter().fold(grid, |tmp_grid, _| {
        do_quarter_of_tilt(
            do_quarter_of_tilt(
                do_quarter_of_tilt(do_quarter_of_tilt(tmp_grid, move_o_left), move_o_left),
                move_o_right,
            ),
            move_o_right,
        )
    })
}

fn do_quarter_of_tilt(grid: Grid, movement: fn(&Vec<Shape>) -> Vec<Shape>) -> Grid {
    let new_grid = change_oder_col_row(grid);

    Grid {
        grid: new_grid.grid.iter().map(|line| movement(line)).collect(),
        order: new_grid.order,
    }
}

fn move_o_right(v: &Vec<Shape>) -> Vec<Shape> {
    let right = true;
    //let v = if right { v.iter().rev() } else { v.iter() };
    let mut f = v
        .iter()
        .rev()
        .fold((vec![], 0, 0), |(mut v, mut rounded, mut empty), shape| {
            match shape {
                Shape::CubeRock => {
                    (0..rounded)
                        .into_iter()
                        .for_each(|_| v.push(Shape::RoundedRock));
                    (0..empty).into_iter().for_each(|_| v.push(Shape::Empty));
                    v.push(Shape::CubeRock);
                    rounded = 0;
                    empty = 0;
                }
                Shape::RoundedRock => rounded += 1,
                Shape::Empty => empty += 1,
            }
            (v, rounded, empty)
        });
    (0..f.1)
        .into_iter()
        .for_each(|_| f.0.push(Shape::RoundedRock));
    (0..f.2).into_iter().for_each(|_| f.0.push(Shape::Empty));
    if right {
        f.0.reverse()
    }
    f.0
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
        let input = include_str!("input1_ex.txt");
        let before = parse_input_col_row(input);
        let computed = do_quarter_of_tilt(before, move_o_left);
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
            assert_eq!(res.order, new_grid.order);
            assert_eq!(res.grid, new_grid.grid);
        }
    }

    fn _helper_print(v: &Grid) -> String {
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
