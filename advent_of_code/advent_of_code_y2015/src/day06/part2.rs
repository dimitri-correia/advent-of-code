use super::common::{parse_line, Instruction};

pub fn part_2_example() -> String {
    let input = "toggle 0,0 through 999,999";
    part_2(input)
}

pub fn part_2_actual_challenge() -> String {
    let input = include_str!("input1.txt");
    part_2(input)
}

fn part_2(input: &str) -> String {
    let mut grid = LightGrid::new();
    input
        .lines()
        .map(|l| parse_line(l))
        .for_each(|(instruction, start, end)| {
            grid.apply_instruction(instruction, start, end);
        });
    grid.count_lights_on().to_string()
}

pub struct LightGrid {
    grid: Vec<Vec<usize>>,
}

impl LightGrid {
    pub fn new() -> Self {
        let mut grid = Vec::new();
        for _ in 0..1000 {
            let mut row = Vec::new();
            for _ in 0..1000 {
                row.push(0);
            }
            grid.push(row);
        }
        Self { grid }
    }

    pub fn apply_instruction(
        &mut self,
        instruction: Instruction,
        start: (usize, usize),
        end: (usize, usize),
    ) {
        for i in start.0..=end.0 {
            for j in start.1..=end.1 {
                match instruction {
                    Instruction::TurnOn => self.grid[i][j] += 1,
                    Instruction::TurnOff => {
                        if self.grid[i][j] > 0 {
                            self.grid[i][j] -= 1
                        }
                    }
                    Instruction::Toggle => self.grid[i][j] += 2,
                };
            }
        }
    }

    pub fn count_lights_on(&self) -> usize {
        self.grid.iter().flatten().sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn actual_challenge() {
        let output = part_2_actual_challenge();        assert_eq!("15343601", output);
    }
    #[test]
    fn example_test() {
        let r = part_2_example();
        assert_eq!("2000000", r);
    }
}
