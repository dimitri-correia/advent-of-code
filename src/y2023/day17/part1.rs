fn part_1(input: &str) -> String {
    let lines = parse_input(input);

    let start_pos = Pos {
        x: 0,
        y: 0,
        consecutive: Consecutive {
            number: 0,
            direction: Dir::Right,
        },
    };
    let mut explored = vec![];
    let min_heat_loss: u32 = u32::MAX;
    let actual_heat_loss: u32 = 0;
    explore_graph(
        &lines,
        start_pos,
        &mut explored,
        actual_heat_loss,
        min_heat_loss,
    )
    .to_string()
}

fn explore_graph(
    lines: &[Vec<u32>],
    pos: Pos,
    explored: &mut Vec<Pos>,
    actual_heat_loss: u32,
    min_heat_loss: u32,
) -> u32 {
    let final_heat_loss = if is_starting_point(pos) {
        actual_heat_loss
    } else {
        actual_heat_loss + lines[pos.x][pos.y]
    };

    if is_ending_point(pos, lines) {
        return if final_heat_loss < min_heat_loss {
            final_heat_loss
        } else {
            min_heat_loss
        };
    }

    if explored.contains(&pos) {
        return u32::MAX;
    }

    explored.push(pos.clone());

    [Dir::Up, Dir::Down, Dir::Right, Dir::Left]
        .iter()
        .map(|dir| {
            let new_x = pos.x as isize + get_dir(dir).0;
            let new_y = pos.y as isize + get_dir(dir).1;
            if outside_graph(&lines, new_x, new_y) {
                return u32::MAX;
            }
            let new_consecutive = get_new_consecutive(&pos, dir);
            if new_consecutive.is_none() {
                return u32::MAX;
            }
            let new_pos = Pos {
                x: new_x as usize,
                y: new_y as usize,
                consecutive: new_consecutive.unwrap(),
            };
            explore_graph(lines, new_pos, explored, final_heat_loss, min_heat_loss)
        })
        .min()
        .unwrap()
}

fn is_starting_point(pos: Pos) -> bool {
    pos.x == 0 && pos.y == 0
}
fn is_ending_point(pos: Pos, lines: &[Vec<u32>]) -> bool {
    pos.x == lines.len() - 1 && pos.y == lines[0].len() - 1
}

fn get_new_consecutive(pos: &Pos, dir: &Dir) -> Option<Consecutive> {
    const MAX_NUMBER: u32 = 3;
    let mut new_number = 0;
    if pos.consecutive.direction == *dir {
        if pos.consecutive.number + 1 == MAX_NUMBER {
            return None;
        }
        new_number = pos.consecutive.number + 1;
    }
    Some(Consecutive {
        number: new_number,
        direction: *dir,
    })
}

fn outside_graph(lines: &[Vec<u32>], x: isize, y: isize) -> bool {
    x < 0 || y < 0 || x as usize > lines.len() - 1 || y as usize > lines[0].len() - 1
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct Pos {
    x: usize,
    y: usize,
    consecutive: Consecutive,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct Consecutive {
    number: u32,
    direction: Dir,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

fn get_dir(dir: &Dir) -> (isize, isize) {
    match dir {
        Dir::Up => (-1, 0),
        Dir::Down => (1, 0),
        Dir::Right => (0, 1),
        Dir::Left => (0, -1),
    }
}

fn parse_input(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn actual_challenge() {
        let input = include_str!("input1.txt");
        let output = part_1(input);
        dbg!(&output);
        assert_eq!("", output);
    }

    #[test]
    fn example_test() {
        let input = include_str!("input1_ex.txt");
        let r = part_1(input);
        assert_eq!("102", r);
    }
}
