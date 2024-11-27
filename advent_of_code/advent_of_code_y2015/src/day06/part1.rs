use super::common::{parse_line, Instruction};

pub fn part_1_actual_challenge() -> String {
    let input = include_str!("input1.txt");
    part_1(input)
}

fn part_1(input: &str) -> String {
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
    grid: Vec<Vec<bool>>,
}

impl LightGrid {
    pub fn new() -> Self {
        let mut grid = Vec::new();
        for _ in 0..1000 {
            let mut row = Vec::new();
            for _ in 0..1000 {
                row.push(false);
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
                    Instruction::TurnOn => self.grid[i][j] = true,
                    Instruction::TurnOff => self.grid[i][j] = false,
                    Instruction::Toggle => self.grid[i][j] = !self.grid[i][j],
                }
            }
        }
    }

    pub fn count_lights_on(&self) -> usize {
        self.grid.iter().flatten().filter(|&&x| x).count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn actual_challenge() {
        let output = part_1_actual_challenge();        
        assert_eq!("400410", output);
    }
}
