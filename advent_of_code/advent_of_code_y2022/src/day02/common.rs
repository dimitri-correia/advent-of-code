use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum GameItems {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

#[derive(Debug, PartialEq, Eq)]
pub enum RoundResult {
    Win = 6,
    Draw = 3,
    Loose = 0,
}

impl GameItems {
    pub fn get_round_result(&self, opponent: &Self) -> RoundResult {
        match (self, opponent) {
            (s, o) if s == o => RoundResult::Draw,
            (GameItems::Paper, GameItems::Rock)
            | (GameItems::Scissors, GameItems::Paper)
            | (GameItems::Rock, GameItems::Scissors) => RoundResult::Win,
            _ => RoundResult::Loose,
        }
    }

    pub fn get_move_for_result(&self, result: &RoundResult) -> Self {
        match (self, result) {
            (_, &RoundResult::Draw) => self.clone(),
            (GameItems::Rock, &RoundResult::Loose) | (GameItems::Paper, &RoundResult::Win) => {
                GameItems::Scissors
            }
            (GameItems::Scissors, &RoundResult::Loose) | (GameItems::Rock, &RoundResult::Win) => {
                GameItems::Paper
            }
            _ => GameItems::Rock,
        }
    }
}

impl FromStr for GameItems {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(GameItems::Rock),
            "B" | "Y" => Ok(GameItems::Paper),
            "C" | "Z" => Ok(GameItems::Scissors),
            _ => unreachable!(),
        }
    }
}

impl FromStr for RoundResult {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(RoundResult::Loose),
            "Y" => Ok(RoundResult::Draw),
            "Z" => Ok(RoundResult::Win),
            _ => unreachable!(),
        }
    }
}
