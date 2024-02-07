use crate::y2023::day14::common::{get_val_col, parse_input_col_row, Shape};

fn part_2(input: &str) -> String {
    const NUMBER_CYCLE: usize = 1_000_000_000;
    let grid = parse_input_col_row(input);
    let final_grid = get_final_grid(grid, NUMBER_CYCLE);

    final_grid
        .into_iter()
        .map(get_val_col)
        .sum::<usize>()
        .to_string()
}

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
    grid.iter().map(|line| move_o_left(line)).collect()
}

fn do_west(grid: Vec<Vec<Shape>>) -> Vec<Vec<Shape>> {
    grid.iter().map(|line| move_o_left(line)).collect()
}

fn do_north(grid: Vec<Vec<Shape>>) -> Vec<Vec<Shape>> {
    grid.iter().map(|line| move_o_right(line)).collect()
}

fn move_o_right(v: &[Shape]) -> Vec<Shape> {
    todo!()
}

fn move_o_left(v: &Vec<Shape>) -> Vec<Shape> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn actual_challenge() {
        let input = include_str!("input1.txt");
        let output = part_2(input);
        dbg!(&output);
        assert_eq!("val", output);
    }

    #[test]
    fn example_test() {
        let input = include_str!("input1_ex.txt"); // same file
        let r = part_2(input);
        assert_eq!("64", r);
    }

    #[test]
    fn example_test_3_cycles() {
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
#...O###.O",
        ];
        let mut new_grid = grid.clone();
        for ex in expected {
            new_grid = get_final_grid(new_grid, 1);
            let res = parse_input_col_row(ex);
            assert_eq!(res, new_grid);
        }
    }
}
