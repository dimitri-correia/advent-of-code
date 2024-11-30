use std::cmp::min;

pub fn count_neighbors_on(grid: &Vec<Vec<bool>>, i: usize, j: usize) -> u8 {
    let mut count = 0;

    for x in i.saturating_sub(1)..=min(i + 1, grid.len() - 1) {
        for y in j.saturating_sub(1)..=min(j + 1, grid[x].len() - 1) {
            if x == i && y == j {
                continue;
            }

            if grid[x][y] {
                count += 1;
            }
        }
    }

    count
}

pub fn parse_input(input: &str) -> Vec<Vec<bool>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '#' => true,
                    '.' => false,
                    _ => panic!("Invalid character in input: {}", c),
                })
                .collect()
        })
        .collect()
}
