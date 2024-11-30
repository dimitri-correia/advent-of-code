use super::common::{count_neighbors_on, parse_input};

pub fn part_1_example() -> String {
    let input = include_str!("input1_ex.txt");
    let nb_steps = 4;
    part_1(input, nb_steps)
}

pub fn part_1_actual_challenge() -> String {
    let input = include_str!("input1.txt");
    let nb_steps = 100;
    part_1(input, nb_steps)
}

fn part_1(input: &str, nb_steps: u8) -> String {
    let mut grid: Vec<Vec<bool>> = parse_input(input);

    for _ in 0..nb_steps {
        grid = step(&grid);
    }

    grid.iter()
        .flat_map(|r| r)
        .filter(|v| **v)
        .count()
        .to_string()
}

fn step(grid: &Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    let mut new_grid = grid.clone();

    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            let nb_on = count_neighbors_on(grid, i, j);
            new_grid[i][j] = match (grid[i][j], nb_on) {
                (true, 2) | (true, 3) => true,
                (false, 3) => true,
                _ => false,
            };
        }
    }

    new_grid
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn actual_challenge() {
        let output = part_1_actual_challenge();
        assert_eq!("768", output);
    }

    #[test]
    fn example_test() {
        let r = part_1_example();
        assert_eq!("4", r);
    }
}
