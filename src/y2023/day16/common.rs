pub fn get_dirs_from_tile(curr_pos: &Position, tile: Tile) -> Vec<Dir> {
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
pub enum Tile {
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
pub enum Dir {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub struct Coord(pub isize, pub isize);

impl Dir {
    pub fn coordinates(&self) -> Coord {
        match self {
            Dir::Up => Coord(-1, 0),
            Dir::Down => Coord(1, 0),
            Dir::Left => Coord(0, -1),
            Dir::Right => Coord(0, 1),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub struct Position {
    pub pos: Coord,
    pub dir: Dir,
}

pub fn parse_input(input: &str) -> Vec<Vec<Tile>> {
    input
        .lines()
        .map(|line| line.chars().map(parse_tile).collect::<Vec<Tile>>())
        .collect()
}
