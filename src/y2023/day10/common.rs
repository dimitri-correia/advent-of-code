pub fn get_path(input: &str) -> (Vec<(usize, usize)>, Vec<Vec<Pipe>>) {
    let (lines_vec, start_pos) = read_input(input);
    let mut pass = vec![start_pos];
    let mut pos = first_pipe(&start_pos, &lines_vec);

    loop {
        pass.push(pos);
        let opt_pos = get_next(&pos, &pass, &lines_vec);

        if opt_pos.is_none() {
            break;
        }

        pos = opt_pos.unwrap();
    }
    (pass, lines_vec)
}

#[derive(Debug, PartialEq, Eq)]
pub enum Pipe {
    // |
    Vertical,
    // -
    Horizontal,
    // L
    NorthEast,
    // J
    NorthWest,
    // 7
    SouthWest,
    // F
    SouthEast,
    // .
    Ground,
    // S
    StartingPosition,
}

pub fn parse_char_to_pipe(c: char) -> Pipe {
    match c {
        '|' => Pipe::Vertical,
        '-' => Pipe::Horizontal,
        'L' => Pipe::NorthEast,
        'J' => Pipe::NorthWest,
        '7' => Pipe::SouthWest,
        'F' => Pipe::SouthEast,
        '.' => Pipe::Ground,
        'S' => Pipe::StartingPosition,
        _ => unreachable!(),
    }
}

pub fn read_input(input: &str) -> (Vec<Vec<Pipe>>, (usize, usize)) {
    let mut lines_vec = vec![];
    let mut start_pos: (usize, usize) = (0, 0);
    for (l_idx, line) in input.lines().enumerate() {
        let mut line_vec = vec![];
        for (c_idx, c) in line.chars().enumerate() {
            let pipe = parse_char_to_pipe(c);
            if pipe == Pipe::StartingPosition {
                start_pos = (l_idx, c_idx)
            }
            line_vec.push(pipe);
        }
        lines_vec.push(line_vec);
    }

    (lines_vec, start_pos)
}

pub fn first_pipe(pos: &(usize, usize), lines_vec: &[Vec<Pipe>]) -> (usize, usize) {
    let mut adjacent_pipes = vec![];
    for &(v, h) in &[(1, 0), (-1, 0), (0, 1), (0, -1)] {
        let x = pos.0 as isize + v;
        let y = pos.1 as isize + h;
        if x < 0 || x > lines_vec.len() as isize || y < 0 || y > lines_vec[0].len() as isize {
            continue;
        }
        let x = x as usize;
        let y = y as usize;
        let pipe = &lines_vec[x][y];
        if pipe == &Pipe::Ground {
            continue;
        }
        let (a, b) = get_two_next_possible_pipes(&&(x, y), &lines_vec);
        if &a == pos || &b == pos {
            adjacent_pipes.push(((x, y), pipe));
        }
    }
    if adjacent_pipes.len() != 2 {
        panic!("more than 2 adjacent pipe at the start");
    }
    adjacent_pipes.pop().unwrap().0
}

pub fn get_next(
    pos: &(usize, usize),
    pass: &[(usize, usize)],
    lines_vec: &[Vec<Pipe>],
) -> Option<(usize, usize)> {
    let (f, l) = get_two_next_possible_pipes(&pos, &lines_vec);
    if pass.contains(&f) {
        if pass.contains(&l) {
            return None;
        }
        return Some(l);
    }
    Some(f)
}

fn get_two_next_possible_pipes(
    pos: &&(usize, usize),
    lines_vec: &&[Vec<Pipe>],
) -> ((usize, usize), (usize, usize)) {
    let pipe = &lines_vec[pos.0][pos.1];
    let (a, b) = get_positions(pipe);
    let f = (
        (a.0 + pos.0 as isize) as usize,
        (a.1 + pos.1 as isize) as usize,
    );
    let l = (
        (b.0 + pos.0 as isize) as usize,
        (b.1 + pos.1 as isize) as usize,
    );
    (f, l)
}

fn get_positions(direction: &Pipe) -> ((isize, isize), (isize, isize)) {
    match direction {
        Pipe::Vertical => ((1, 0), (-1, 0)),
        Pipe::Horizontal => ((0, 1), (0, -1)),
        Pipe::NorthEast => ((-1, 0), (0, 1)),
        Pipe::NorthWest => ((-1, 0), (0, -1)),
        Pipe::SouthWest => ((1, 0), (0, -1)),
        Pipe::SouthEast => ((1, 0), (0, 1)),
        _ => unreachable!(),
    }
}
