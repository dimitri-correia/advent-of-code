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

pub trait GetRoundResult {
    fn get_round_result(&self, opponent: &Self) -> RoundResult;
    fn get_move_for_result(&self, result: &RoundResult) -> Self;
}

impl GetRoundResult for GameItems {
    fn get_round_result(&self, opponent: &Self) -> RoundResult {
        if opponent == self {
            RoundResult::Draw
        } else if let (GameItems::Paper, GameItems::Rock)
        | (GameItems::Scissors, GameItems::Paper)
        | (GameItems::Rock, GameItems::Scissors) = (opponent, self)
        {
            RoundResult::Loose
        } else {
            RoundResult::Win
        }
    }

    fn get_move_for_result(&self, result: &RoundResult) -> Self {
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
