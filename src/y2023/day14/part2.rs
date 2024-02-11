use crate::y2023::day14::common::{
    change_order_col_row, get_val_col, parse_input_col_row, Grid, Shape,
};
use std::hash::{DefaultHasher, Hash, Hasher};

fn part_2(input: &str) -> String {
    const NUMBER_CYCLE: usize = 1_000; // 1_000_000_000;

    // as first element of cycle is north, we need to have the first grid in row_col format
    let mut grid = parse_input_col_row(input);

    // also in row_col format
    get_final_grid(&mut grid, NUMBER_CYCLE);

    grid.grid
        .into_iter()
        .map(get_val_col) // todo
        .sum::<usize>()
        .to_string()
}

fn get_final_grid(grid: &mut Grid, number_cycle: usize) {
    let mut hashes = vec![calculate_hash(&grid)];

    for already_done_cycle in 0..number_cycle {
        do_quarter_of_tilt(grid, move_o_right);
        do_quarter_of_tilt(grid, move_o_right);
        do_quarter_of_tilt(grid, move_o_left);
        do_quarter_of_tilt(grid, move_o_left);
        let hash = calculate_hash(&grid);
        let a = hashes
            .iter()
            .enumerate()
            .find_map(|(idx, h)| if h == &hash { Some(idx) } else { None });
        if a.is_some() {
            dbg!(&hashes);
            //let remaining = number_cycle - already_done_cycle;
            return;
        }
        hashes.push(hash);
    }
}

fn calculate_hash(g: &Grid) -> u64 {
    let mut h = DefaultHasher::new();
    g.hash(&mut h);
    h.finish()
}

fn do_quarter_of_tilt(grid: &mut Grid, movement: fn(&Vec<Shape>) -> Vec<Shape>) {
    change_order_col_row(grid);

    grid.grid = grid.grid.iter().map(|line| movement(line)).collect();
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
    use crate::y2023::day14::common::{parse_char, parse_input_row_col, Order};
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
        let mut after = before.clone();
        do_quarter_of_tilt(&mut after, move_o_left);
        assert_eq!(expected, after);
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
        let mut grid = parse_input_col_row(input);
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
        for ex in expected {
            get_final_grid(&mut grid, 1);
            let res = parse_input_col_row(ex);
            assert_eq!(res.order, grid.order);
            assert_eq!(res.grid, grid.grid);
        }
    }

    fn _helper_print(v: &Grid) -> String {
        let mut grid = v.clone();
        fn inverse_parse(s: &Shape) -> char {
            match s {
                Shape::RoundedRock => 'O',
                Shape::Empty => '.',
                Shape::CubeRock => '#',
            }
        }
        if grid.order == Order::RowCol {
            change_order_col_row(&mut grid)
        };
        let p = grid
            .grid
            .iter()
            .map(|row| row.iter().map(|s| inverse_parse(s)).collect::<String>())
            .join("\n");

        dbg!(p)
    }
}
