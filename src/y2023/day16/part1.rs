use std::collections::HashSet;
use std::vec;

fn part_1(input: &str) -> String {
    let grid = parse_input(input);

    let mut energized: HashSet<Position> = HashSet::new();

    let coord = Coord(0, 0);

    get_dirs_from_tile(
        &Position {
            pos: coord,
            dir: Dir::Right,
        },
        grid[0][0],
    )
    .iter()
    .for_each(|&dir| {
        let pos = Position { pos: coord, dir };
        follow_path(&grid, &mut energized, pos);
    });

    energized
        .iter()
        .map(|p| p.pos)
        .collect::<HashSet<Coord>>()
        .len()
        .to_string()
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

fn get_dirs_from_tile(curr_pos: &Position, tile: Tile) -> Vec<Dir> {
    let new_dir = match tile {
        Tile::Empty => vec![curr_pos.dir],
        Tile::MirrorLR => match curr_pos.dir {
            Dir::Up => vec![Dir::Right],
            Dir::Down => vec![Dir::Left],
            Dir::Left => vec![Dir::Down],
            Dir::Right => vec![Dir::Up],
        },
        Tile::MirrorRL => match curr_pos.dir {
            Dir::Up => vec![Dir::Left],
            Dir::Down => vec![Dir::Right],
            Dir::Left => vec![Dir::Up],
            Dir::Right => vec![Dir::Down],
        },
        Tile::SplitterH => match curr_pos.dir {
            Dir::Up | Dir::Down => vec![Dir::Right, Dir::Left],
            _ => vec![curr_pos.dir],
        },
        Tile::SplitterV => match curr_pos.dir {
            Dir::Left | Dir::Right => vec![Dir::Up, Dir::Down],
            _ => vec![curr_pos.dir],
        },
    };
    new_dir
}

#[derive(Debug, Copy, Clone)]
enum Tile {
    Empty,
    MirrorLR,
    MirrorRL,
    SplitterH,
    SplitterV,
}

fn parse_tile(c: char) -> Tile {
    match c {
        '.' => Tile::Empty,
        '/' => Tile::MirrorLR,
        '\\' => Tile::MirrorRL,
        '|' => Tile::SplitterV,
        '-' => Tile::SplitterH,
        _ => panic!(),
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
struct Coord(isize, isize);

impl Dir {
    fn coordinates(&self) -> Coord {
        match self {
            Dir::Up => Coord(-1, 0),
            Dir::Down => Coord(1, 0),
            Dir::Left => Coord(0, -1),
            Dir::Right => Coord(0, 1),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
struct Position {
    pos: Coord,
    dir: Dir,
}

fn parse_input(input: &str) -> Vec<Vec<Tile>> {
    input
        .lines()
        .map(|line| line.chars().map(parse_tile).collect::<Vec<Tile>>())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn actual_challenge() {
        let input = include_str!("input1.txt");
        let output = part_1(input);
        dbg!(&output);
        assert_eq!("7798", output);
    }

    #[test]
    fn example_test() {
        let input = include_str!("input1_ex.txt");
        let r = part_1(input);
        assert_eq!("46", r);
    }
}
