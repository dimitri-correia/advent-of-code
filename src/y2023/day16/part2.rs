use crate::y2023::day16::common::{get_dirs_from_tile, parse_input, Coord, Dir, Position, Tile};
use std::collections::HashSet;

fn part_2(input: &str) -> String {
    let grid = parse_input(input);

    (0..grid.len())
        .flat_map(|x| {
            (0..grid[0].len())
                .filter_map(|y| {
                    if x == 0 || y == 0 || x == grid.len() - 1 || y == grid[0].len() - 1 {
                        let dir = if x == 0 {
                            Dir::Right
                        } else if y == 0 {
                            Dir::Down
                        } else if x == grid.len() - 1 {
                            Dir::Left
                        } else {
                            Dir::Up
                        };
                        Some((Coord(x as isize, y as isize), dir))
                    } else {
                        None
                    }
                })
                .collect::<Vec<(Coord, Dir)>>()
        })
        .inspect(|(pos, dir)| {
            dbg!(&pos, &dir);
        })
        .map(|(pos, dir)| {
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

fn generate_all_starts

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
}
