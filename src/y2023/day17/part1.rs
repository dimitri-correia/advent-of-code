use std::collections::HashMap;

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

    let mut explored = HashMap::new();
    let mut min_heat_loss: u32 = u32::MAX;

    explore_graph(
        &lines,
        start_pos,
        &mut explored,
        0,
        &mut min_heat_loss,
        vec![],
    );

    (min_heat_loss - lines[0][0]).to_string()
}

fn explore_graph(
    lines: &[Vec<u32>],
    pos: Pos,
    explored: &mut HashMap<Pos, u32>,
    actual_heat_loss: u32,
    min_heat_loss: &mut u32,
    road: Vec<String>,
) {
    let final_heat_loss = actual_heat_loss + lines[pos.x][pos.y];

    if final_heat_loss >= *min_heat_loss {
        return;
    }

    if is_ending_point(pos, lines) {
        let mut new_road = road.clone();
        new_road.push(format!(
            "{} - {} - {} - {}",
            pos.x, pos.y, lines[pos.x][pos.y], final_heat_loss
        ));
        dbg!(&final_heat_loss);
        dbg!(&new_road);
        helper_print(new_road, lines);

        *min_heat_loss = final_heat_loss;
        return;
    }

    if let Some(old_min) = explored.get(&pos) {
        if old_min <= &final_heat_loss {
            return;
        }
    }

    explored.insert(pos.clone(), final_heat_loss);

    [Dir::Right, Dir::Down, Dir::Up, Dir::Left]
        .iter()
        .for_each(|dir| {
            let new_x = pos.x as isize + get_dir(dir).0;
            let new_y = pos.y as isize + get_dir(dir).1;
            if outside_graph(&lines, new_x, new_y) {
                return;
            }
            let new_consecutive = get_new_consecutive(&pos, dir);
            if new_consecutive.is_none() {
                return;
            }
            let new_pos = Pos {
                x: new_x as usize,
                y: new_y as usize,
                consecutive: new_consecutive.unwrap(),
            };
            let mut new_road = road.clone();
            new_road.push(format!(
                "{} - {} - {} - {}",
                pos.x, pos.y, lines[pos.x][pos.y], final_heat_loss
            ));
            explore_graph(
                lines,
                new_pos,
                explored,
                final_heat_loss,
                min_heat_loss,
                new_road,
            )
        });
}

fn helper_print(road: Vec<String>, grid: &[Vec<u32>]) {
    let positions: Vec<(i32, i32)> = road
        .iter()
        .map(|l| {
            let numbers: Vec<_> = l
                .split('-')
                .map(|s| s.trim().parse::<i32>().unwrap())
                .collect();
            (numbers[0], numbers[1])
        })
        .collect();

    for (i, row) in grid.iter().enumerate() {
        for (j, &cell) in row.iter().enumerate() {
            let mut replaced = false;
            for (x, y) in positions.iter() {
                if (i as i32, j as i32) == (*x, *y) {
                    print!("* ");
                    replaced = true;
                    break;
                }
            }
            if !replaced {
                print!("{} ", cell);
            }
        }
        println!();
    }
}

fn is_ending_point(pos: Pos, lines: &[Vec<u32>]) -> bool {
    pos.x == lines.len() - 1 && pos.y == lines[0].len() - 1
}

fn get_new_consecutive(pos: &Pos, dir: &Dir) -> Option<Consecutive> {
    const MAX_NUMBER: u32 = 3;
    if pos.consecutive.direction != *dir {
        return Some(Consecutive {
            number: 1,
            direction: *dir,
        });
    }
    if pos.consecutive.number + 1 > MAX_NUMBER {
        return None;
    }
    Some(Consecutive {
        number: pos.consecutive.number + 1,
        direction: *dir,
    })
}

fn outside_graph(lines: &[Vec<u32>], x: isize, y: isize) -> bool {
    x < 0 || y < 0 || x as usize > lines.len() - 1 || y as usize > lines[0].len() - 1
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
struct Pos {
    x: usize,
    y: usize,
    consecutive: Consecutive,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
struct Consecutive {
    number: u32,
    direction: Dir,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
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
