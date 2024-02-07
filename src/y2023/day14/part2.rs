use crate::y2023::day14::common::{change_oder_col_row, get_val_col, parse_input_row_col, Shape};

fn part_2(input: &str) -> String {
    const NUMBER_CYCLE: usize = 1_000_000_000;

    // as first element of cycle is north, we need to have the first grid in row_col format
    let grid = parse_input_row_col(input);

    // also in row_col format
    let final_grid = get_final_grid(grid, NUMBER_CYCLE);

    final_grid
        .into_iter()
        .map(get_val_col) // todo
        .sum::<usize>()
        .to_string()
}

///
/// take a grid in the row_col format and cycle over it `number_cycle` times to return it in the same format
///
fn get_final_grid(grid: Vec<Vec<Shape>>, number_cycle: usize) -> Vec<Vec<Shape>> {
    let final_grid = (0..number_cycle).into_iter().fold(grid, |tmp_grid, _| {
        // maybe optimize and stop if same grid as before
        do_east(do_south(do_west(do_north(tmp_grid))))
    });
    final_grid
}

fn do_east(grid: Vec<Vec<Shape>>) -> Vec<Vec<Shape>> {
    grid.iter().map(|line| move_o_right(line)).collect()
}

fn do_south(grid: Vec<Vec<Shape>>) -> Vec<Vec<Shape>> {
    change_oder_col_row(grid)
        .iter()
        .map(|line| move_o_left(line))
        .collect()
}

fn do_west(grid: Vec<Vec<Shape>>) -> Vec<Vec<Shape>> {
    grid.iter().map(|line| move_o_left(line)).collect()
}

fn do_north(grid: Vec<Vec<Shape>>) -> Vec<Vec<Shape>> {
    change_oder_col_row(grid)
        .iter()
        .map(|line| move_o_right(line))
        .collect()
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
    use crate::y2023::day14::common::parse_char;

    // #[test]
    // fn actual_challenge() {
    //     let input = include_str!("input1.txt");
    //     let output = part_2(input);
    //     dbg!(&output);
    //     assert_eq!("val", output);
    // }
    //
    // #[test]
    // fn example_test() {
    //     let input = include_str!("input1_ex.txt"); // same file
    //     let r = part_2(input);
    //     assert_eq!("64", r);
    // }

    #[test]
    fn test_do_north() {
        let input = include_str!("input1.txt");
        let before = parse_input_row_col(input);
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
        let grid = parse_input_row_col(input);
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
#...O###.O",
        ];
        let mut new_grid = grid.clone();
        for ex in expected {
            new_grid = get_final_grid(new_grid, 1);
            let res = parse_input_row_col(ex);
            assert_eq!(res, new_grid);
        }
    }
}
