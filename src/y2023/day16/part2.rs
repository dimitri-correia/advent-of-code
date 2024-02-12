use crate::y2023::day16::common::{get_number_energized_tiles, parse_input, Coord, Dir, Tile};

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
        let input = include_str!("input1.txt");
        let output = part_2(input);
        dbg!(&output);
        assert_eq!("val", output);
    }

    #[test]
    fn example_test() {
        let input = include_str!("input1_ex.txt");
        let r = part_2(input);
        assert_eq!("51", r);
    }

    #[test]
    fn test_generate_all_starts() {
        let grid = parse_input(include_str!("input1.txt"));
        let all_starts = generate_all_starts(&grid);
        assert_eq!(110 * 4, all_starts.len());
    }
}
