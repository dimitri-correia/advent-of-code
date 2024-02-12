use crate::y2023::day16::common::{get_dirs_from_tile, parse_input, Coord, Dir, Position, Tile};
use std::collections::HashSet;

fn part_2(input: &str) -> String {
    let grid = parse_input(input);

    generate_all_starts(&grid)
        .iter()
        .map(|&(dir, pos)| {
            let mut energized: HashSet<Position> = HashSet::new();
            get_dirs_from_tile(&Position { pos, dir }, grid[0][0])
                .iter()
                .for_each(|&dir| {
                    let pos = Position { pos, dir };
                    follow_path(&grid, &mut energized, pos);
                });
            energized
                .iter()
                .map(|p| p.pos)
                .collect::<HashSet<Coord>>()
                .len()
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

fn follow_path(grid: &[Vec<Tile>], energized: &mut HashSet<Position>, pos: Position) {
    if energized.contains(&pos) {
        return;
    }
    energized.insert(pos);
    get_next(&pos, &grid)
        .iter()
        .for_each(|&next_pos| follow_path(grid, energized, next_pos));
}

fn get_next(curr_pos: &Position, grid: &[Vec<Tile>]) -> Vec<Position> {
    let x = curr_pos.pos.0 + curr_pos.dir.coordinates().0;
    let y = curr_pos.pos.1 + curr_pos.dir.coordinates().1;
    if x < 0 || x as usize >= grid.len() || y < 0 || y as usize >= grid[0].len() {
        return vec![];
    }
    let new_cord = Coord(x, y);
    let tile = grid[x as usize][y as usize];

    let new_dir = get_dirs_from_tile(curr_pos, tile);

    new_dir
        .iter()
        .map(|dir| Position {
            pos: new_cord,
            dir: *dir,
        })
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
}
