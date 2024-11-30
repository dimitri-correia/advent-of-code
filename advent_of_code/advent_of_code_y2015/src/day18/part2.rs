use super::common::{count_neighbors_on, parse_input};

pub fn part_2_example() -> String {
    let input = include_str!("input1_ex.txt");
    let nb_steps = 5;
    part_2(input, nb_steps)
}

pub fn part_2_actual_challenge() -> String {
    let input = include_str!("input1.txt");
    let nb_steps = 100;
    part_2(input, nb_steps)
}

fn part_2(input: &str, nb_steps: u8) -> String {
    let mut grid: Vec<Vec<bool>> = parse_input(input);

    // update 4 corners
    let max_j = grid[0].len() - 1;
    let max_i = grid.len() - 1;
    grid[0][0] = true;
    grid[0][max_j] = true;
    grid[max_i][0] = true;
    grid[max_i][max_j] = true;

    for _ in 0..nb_steps {
        grid = step(&grid);
        println!("{:?}", grid);
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
            if [
                (0, 0),
                (0, grid[0].len() - 1),
                (grid.len() - 1, 0),
                (grid.len() - 1, grid[0].len() - 1),
            ]
            .contains(&(i, j))
            {
                continue;
            }
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
        let output = part_2_actual_challenge();
        assert_eq!("781", output);
    }
    #[test]
    fn example_test() {
        let r = part_2_example();
        assert_eq!("17", r);
    }
}
