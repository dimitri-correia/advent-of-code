fn part_2(input: &str, debug_print: bool) -> String {
    let grid = parse_input(input);

    let (starting_point, ending_point) = get_start_end(&grid);

    let max_path = follow_path(starting_point, ending_point, grid.clone()).unwrap();

    if debug_print {
        print_final_grid(&grid, &max_path);
    }

    max_path.len().to_string()
}

fn follow_path(actual_pos: Pos, ending_point: Pos, grid: Vec<Vec<Type>>) -> Option<Vec<Pos>> {
    // recursive was exceeding stack
    let mut stack = vec![];

    stack.push((actual_pos, vec![], vec![]));

    let mut res = vec![];

    while let Some((current_pos, current_path, visited)) = stack.pop() {
        if current_pos == ending_point {
            res.push(current_path.clone());
            continue;
        }
        if visited.contains(&current_pos) {
            continue;
        }

        let mut new_visited = visited.clone();
        let mut new_path = current_path.clone();
        new_visited.push(current_pos);
        new_path.push(current_pos);

        get_next(&current_pos, &grid).iter().for_each(|&pos| {
            stack.push((pos, new_path.clone(), new_visited.clone()));
        });
    }

    res.into_iter().max_by_key(|p| p.len())
}

fn print_final_grid(grid: &Vec<Vec<Type>>, path: &Vec<Pos>) {
    for (y, row) in grid.iter().enumerate() {
        for (x, tile) in row.iter().enumerate() {
            let pos = Pos {
                x: x as isize,
                y: y as isize,
            };
            if path.contains(&pos) {
                print!("O");
            } else {
                match tile {
                    Type::Forest => print!("#"),
                    Type::Path => print!("."),
                    Type::Slopes(_) => print!(".",),
                }
            }
        }
        println!();
    }
}

fn get_next_if_slopes(pos: &Pos, slope_type: &SlopeType) -> Vec<Pos> {
    // assuming a slope can't lead outside of grid nor into a # (Forest)
    let dir = match slope_type {
        SlopeType::Up => (0, -1),
        SlopeType::Down => (0, 1),
        SlopeType::Left => (-1, 0),
        SlopeType::Right => (1, 0),
    };
    vec![pos.move_dir(&dir)]
}

fn get_next(pos: &Pos, grid: &Vec<Vec<Type>>) -> Vec<Pos> {
    [(0, 1), (0, -1), (1, 0), (-1, 0)]
        .iter()
        .filter_map(|mov| {
            let potential_pos = pos.move_dir(mov);
            if is_pos_outside(potential_pos, &grid) || potential_pos.get_type(grid) == &Type::Forest
            {
                None
            } else {
                Some(potential_pos)
            }
        })
        .collect()
}

fn is_pos_outside(pos: Pos, grid: &&Vec<Vec<Type>>) -> bool {
    pos.x < 0 || pos.y < 0 || pos.y >= grid.len() as isize || pos.x >= grid[0].len() as isize
}

fn get_start_end(grid: &Vec<Vec<Type>>) -> (Pos, Pos) {
    let start_pos = grid[0].iter().position(|v| v == &Type::Path).unwrap();
    let end_pos = grid
        .last()
        .unwrap()
        .iter()
        .position(|v| v == &Type::Path)
        .unwrap();

    (
        Pos {
            x: start_pos as isize,
            y: 0,
        },
        Pos {
            x: end_pos as isize,
            y: grid.len() as isize - 1,
        },
    )
}

fn parse_input(input: &str) -> Vec<Vec<Type>> {
    input
        .lines()
        .map(|l| l.chars().map(|c| Type::from_char(c)).collect())
        .collect()
}

#[derive(Debug, Eq, PartialEq, Clone, Copy, Hash)]
struct Pos {
    x: isize,
    y: isize,
}

impl Pos {
    fn move_dir(&self, mov: &(isize, isize)) -> Self {
        Pos {
            x: self.x + mov.0,
            y: self.y + mov.1,
        }
    }

    fn get_type(self, grid: &Vec<Vec<Type>>) -> &Type {
        &grid[self.y as usize][self.x as usize]
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
enum Type {
    Path,
    Forest,
    Slopes(SlopeType),
}

#[derive(Debug, Eq, PartialEq, Clone)]
enum SlopeType {
    Up,
    Down,
    Right,
    Left,
}

impl Type {
    fn from_char(c: char) -> Self {
        match c {
            '#' => Type::Forest,
            '.' => Type::Path,
            '>' => Type::Slopes(SlopeType::Right),
            '<' => Type::Slopes(SlopeType::Left),
            '^' => Type::Slopes(SlopeType::Up),
            'v' => Type::Slopes(SlopeType::Down),
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn actual_challenge() {
        let input = include_str!("input1.txt");
        let output = part_2(input, false);
        
        assert_eq!("", output);
    }
    #[test]
    fn example_test() {
        let input = include_str!("input1_ex.txt");
        let r = part_2(input, false);
        assert_eq!("154", r);
    }
}
