fn main() {
    let input = include_str!("./input1.txt");
    let output = part_1(input);
    dbg!(output);
}

fn part_1(input: &str) -> String {
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

    dbg!(start_pos);

    let mut pass = vec![start_pos];
    let mut pos = first_pipe(&start_pos, &lines_vec);

    dbg!(&pos);
    dbg!(&pass);

    let mut i = 0;
    loop {
        pass.push(pos);
        pos = get_next(&pos, &pass, &lines_vec);
        dbg!(&pos);

        if pos == start_pos {
            break;
        }
        i += 1;
        if i == 4 {
            break;
        }
    }
    dbg!(pass);

    "".to_string()
}

#[derive(Debug, PartialEq, Eq)]
enum Pipe {
    Vertical,
    // |
    Horizontal,
    // -
    NorthEast,
    // L
    NorthWest,
    // J
    SouthWest,
    // 7
    SouthEast,
    // F
    Ground,
    // .
    StartingPosition, // S
}

fn get_positions(direction: &Pipe) -> ((isize, isize), (isize, isize)) {
    match direction {
        Pipe::Vertical => ((1, 0), (-1, 0)),
        Pipe::Horizontal => ((0, 1), (0, -1)),
        Pipe::NorthEast => ((-1, 0), (0, 1)),
        Pipe::NorthWest => ((-1, 0), (0, -1)),
        Pipe::SouthWest => ((1, 0), (0, -1)),
        Pipe::SouthEast => ((1, 0), (0, 1)),
        _ => panic!(),
    }
}

fn parse_char_to_pipe(c: char) -> Pipe {
    match c {
        '|' => Pipe::Vertical,
        '-' => Pipe::Horizontal,
        'L' => Pipe::NorthEast,
        'J' => Pipe::NorthWest,
        '7' => Pipe::SouthWest,
        'F' => Pipe::SouthEast,
        '.' => Pipe::Ground,
        'S' => Pipe::StartingPosition,
        _ => panic!(),
    }
}

fn first_pipe(pos: &(usize, usize), lines_vec: &[Vec<Pipe>]) -> (usize, usize) {
    for &(v, h) in &[
        (1, 0),
        (-1, 0),
        (0, 1),
        (0, -1),
        (1, 1),
        (1, -1),
        (-1, 1),
        (-1, -1),
    ] {
        let x = (pos.0 as isize + v) as usize;
        let y = (pos.1 as isize + h) as usize;
        let pipe = &lines_vec[x][y];
        if pipe == &Pipe::Ground {
            continue;
        }
        return (x, y);
    }
    *pos
}

fn get_next(
    pos: &(usize, usize),
    pass: &[(usize, usize)],
    lines_vec: &[Vec<Pipe>],
) -> (usize, usize) {
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
    dbg!(f, l);
    dbg!(pass.last());
    if f == *pass.last().unwrap() {
        l
    } else {
        f
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works1() {
        let input = include_str!("./input1_ex.txt");
        let r = part_1(input);
        assert_eq!("4", r);
    }

    #[test]
    fn it_works2() {
        let input = include_str!("./input1_ex2.txt");
        let r = part_1(input);
        assert_eq!("4", r);
    }

    #[test]
    fn it_works3() {
        let input = include_str!("./input1_ex3.txt");
        let r = part_1(input);
        assert_eq!("8", r);
    }

    #[test]
    fn it_works4() {
        let input = include_str!("./input1_ex4.txt");
        let r = part_1(input);
        assert_eq!("8", r);
    }
}
