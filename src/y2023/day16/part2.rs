use crate::y2023::day16::common::{get_number_energized_tiles, parse_input, Coord, Dir, Tile};

fn part_2(input: &str) -> String {
    let grid = parse_input(input);

    generate_all_starts(&grid)
        .iter()
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
                (Dir::Down, Coord(index as isize, 0)),
                (Dir::Up, Coord(index as isize, row_count as isize - 1)),
            ]
        })
        .chain((0..col_count).flat_map(|index| {
            [
                (Dir::Left, Coord(col_count as isize - 1, index as isize)),
                (Dir::Right, Coord(0, index as isize)),
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
    fn test_generate_all_starts_small() {
        let grid = parse_input("...\n...\n...");
        let all_starts = generate_all_starts(&grid);
        assert_eq!(12, all_starts.len());
        assert!([
            (Dir::Down, Coord(0, 0)),
            (Dir::Right, Coord(0, 0)),
            (Dir::Right, Coord(0, 1)),
            (Dir::Right, Coord(0, 2)),
            (Dir::Up, Coord(0, 2)),
            (Dir::Up, Coord(1, 2)),
            (Dir::Up, Coord(2, 2)),
            (Dir::Left, Coord(2, 2)),
            (Dir::Left, Coord(2, 1)),
            (Dir::Left, Coord(2, 0)),
            (Dir::Down, Coord(2, 0)),
            (Dir::Down, Coord(1, 0)),
        ]
        .iter()
        .inspect(|dc| {
            dbg!(&dc);
        })
        .all(|dc| all_starts.contains(dc)))
    }

    #[test]
    fn test_generate_all_starts_example() {
        let grid = parse_input(include_str!("input1_ex.txt"));
        let all_starts = generate_all_starts(&grid);
        assert_eq!(40, all_starts.len());
    }
}
