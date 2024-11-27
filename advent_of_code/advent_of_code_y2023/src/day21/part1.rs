fn part_1(input: &str, steps_remaining: i32, print_grid: bool) -> String {
    let (start_position, grid) = parse_input(input);

    let mut actual_pos = vec![];
    actual_pos.push(start_position);

    (0..steps_remaining).for_each(|_| {
        let size = actual_pos.len();
        (0..size).for_each(|_| {
            let pos = actual_pos.remove(0);
            get_next(pos, &grid, &mut actual_pos);
        })
    });

    if print_grid {
        print_final_grid(&grid, &actual_pos);
    }

    actual_pos.len().to_string()
}

fn print_final_grid(grid: &Vec<Vec<Tile>>, possible_pos: &Vec<Position>) {
    for (x, row) in grid.iter().enumerate() {
        for (y, tile) in row.iter().enumerate() {
            let pos = Position {
                x: x as isize,
                y: y as isize,
            };
            if possible_pos.contains(&pos) {
                print!("O");
            } else {
                match tile {
                    Tile::Rock => print!("#"),
                    Tile::GardenPlot => print!("."),
                    _ => unreachable!(),
                }
            }
        }
        println!();
    }
}

fn get_next(pos: Position, grid: &Vec<Vec<Tile>>, actual_pos: &mut Vec<Position>) {
    [(0, 1), (0, -1), (1, 0), (-1, 0)].iter().for_each(|mov| {
        let potential_pos = pos.move_dir(mov);
        if is_pos_outside(potential_pos, &grid) || !is_walkable_position(grid, &potential_pos) {
            return;
        }
        if !actual_pos.contains(&potential_pos) {
            actual_pos.push(potential_pos);
        }
    });
}

fn is_walkable_position(grid: &Vec<Vec<Tile>>, potential_pos: &Position) -> bool {
    Tile::GardenPlot == grid[potential_pos.x as usize][potential_pos.y as usize]
}

fn is_pos_outside(pos: Position, grid: &&Vec<Vec<Tile>>) -> bool {
    pos.x < 0 || pos.x >= grid.len() as isize || pos.y < 0 || pos.y >= grid[0].len() as isize
}

fn parse_input(input: &str) -> (Position, Vec<Vec<Tile>>) {
    let mut start = None;
    let grid = input
        .lines()
        .enumerate()
        .map(|(x, l)| {
            l.chars()
                .enumerate()
                .map(|(y, c)| match c {
                    '#' => Tile::Rock,
                    '.' => Tile::GardenPlot,
                    'S' => {
                        start = Some(Position {
                            x: x as isize,
                            y: y as isize,
                        });
                        Tile::GardenPlot
                    }
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect();
    (start.unwrap(), grid)
}

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
struct Position {
    x: isize,
    y: isize,
}

impl Position {
    fn move_dir(&self, mov: &(isize, isize)) -> Self {
        Position {
            x: self.x + mov.0,
            y: self.y + mov.1,
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
enum Tile {
    Rock,
    Start,
    GardenPlot,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn actual_challenge() {
        let input = include_str!("input1.txt");
        let output = part_1(input, 64, false);
        
        assert_eq!("3574", output);
    }

    #[test]
    fn example_test() {
        let input = include_str!("input1_ex.txt");
        let r = part_1(input, 6, false);
        assert_eq!("16", r);
    }
}
