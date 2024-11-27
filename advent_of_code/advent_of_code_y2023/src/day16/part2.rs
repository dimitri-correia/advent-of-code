use crate::day16::common::{get_number_energized_tiles, parse_input, Coord, Dir, Tile};

pub fn part_2_example() -> String {
    let input = include_str!("input1_ex.txt");
    part_2(input)
}

pub fn part_2_actual_challenge() -> String {
    let input = include_str!("input1.txt");
    part_2(input)
}

fn part_2(input: &str) -> String {
    let grid = parse_input(input);

    generate_all_starts(&grid)
        .iter()
        .inspect(|a| {
            dbg!(a);
        })
        .map(|&(dir, pos)| get_number_energized_tiles(&grid, dir, pos))
        .inspect(|a| {
            dbg!(a);
        })
        .max()
        .unwrap()
        .to_string()
}

fn generate_all_starts(grid: &[Vec<Tile>]) -> Vec<(Dir, Coord)> {
    let row_count = grid.len();
    let col_count = grid[0].len();
    (0..row_count)
        .flat_map(|index| {
            [
                (Dir::Right, Coord(index as isize, 0)),
                (Dir::Left, Coord(index as isize, row_count as isize - 1)),
            ]
        })
        .chain((0..col_count).flat_map(|index| {
            [
                (Dir::Up, Coord(col_count as isize - 1, index as isize)),
                (Dir::Down, Coord(0, index as isize)),
            ]
        }))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn actual_challenge() {
        let output = part_2_actual_challenge();        assert_eq!("8026", output);
    }

    #[test]
    fn example_test() {
        let r = part_2_example();
        assert_eq!("51", r);
    }

    #[test]
    fn test_generate_all_starts() {
        let grid = parse_input(include_str!("input1.txt"));
        let all_starts = generate_all_starts(&grid);
        assert_eq!(110 * 4, all_starts.len());
    }
}
