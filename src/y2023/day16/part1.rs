use std::collections::HashSet;
use std::vec;

fn part_1(input: &str) -> String {
    let grid = parse_input(input);

    let mut energized: HashSet<Position> = HashSet::new();
    let mut pos: HashSet<Position> = HashSet::new();

    pos.insert(Position {
        pos: Coord(0, 0),
        dir: Dir::Right,
    });

    while let Some(curr_pos) = pos.iter().cloned().next() {
        pos.remove(&curr_pos);
        if energized.contains(&curr_pos) {
            continue;
        }

        get_next(&curr_pos, &grid).iter().for_each(|n| {
            pos.insert(*n);
        });

        energized.insert(curr_pos);
    }

    dbg!(&energized);

    energized
        .iter()
        .map(|p| p.pos)
        .collect::<HashSet<Coord>>()
        .len()
        .to_string()
}

fn get_next(curr_pos: &Position, grid: &[Vec<Tile>]) -> Vec<Position> {
    let x = curr_pos.pos.0 + curr_pos.dir.coordinates().0;
    let y = curr_pos.pos.1 + curr_pos.dir.coordinates().1;
    if x < 0 || x as usize >= grid.len() || y < 0 || y as usize >= grid[0].len() {
        return vec![];
    }
    let new_cord = Coord(x, y);
    let tile = grid[x as usize][y as usize];

    dbg!(&curr_pos);
    dbg!(&tile);

    let new_dir = match tile {
        Tile::Empty => vec![curr_pos.dir],
        Tile::MirrorBLUR => match curr_pos.dir {
            Dir::Up => vec![Dir::Right],
            Dir::Down => vec![Dir::Left],
            Dir::Left => vec![Dir::Up],
            Dir::Right => vec![Dir::Down],
        },
        Tile::MirrorULBR => match curr_pos.dir {
            Dir::Up => vec![Dir::Left],
            Dir::Down => vec![Dir::Right],
            Dir::Left => vec![Dir::Down],
            Dir::Right => vec![Dir::Up],
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
        .iter()
        .map(|dir| Position {
            pos: new_cord,
            dir: *dir,
        })
        .collect()
}

#[derive(Debug, Copy, Clone)]
enum Tile {
    Empty,
    MirrorBLUR,
    // / bottom left upper right
    MirrorULBR,
    // \ upper left bottom right
    SplitterH,
    // | horizontal
    SplitterV, // - vertical
}

fn parse_tile(c: char) -> Tile {
    match c {
        '.' => Tile::Empty,
        '/' => Tile::MirrorBLUR,
        '\\' => Tile::MirrorULBR,
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
    let mut grid = vec![];
    for line in input.lines() {
        let mut row = vec![];
        for c in line.chars() {
            row.push(parse_tile(c));
        }
        grid.push(row);
    }
    grid
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn actual_challenge() {
        let input = include_str!("input1.txt");
        let output = part_1(input);
        dbg!(&output);
        assert_eq!("val", output);
    }

    #[test]
    fn example_test() {
        let input = include_str!("input1_ex.txt");
        let r = part_1(input);
        assert_eq!("46", r);
    }
}
