#[derive(PartialEq, Eq, Hash, Debug, Copy, Clone)]
pub enum Colors {
    R,
    G,
    B,
}

pub struct GameInfo {
    pub game_id: u32,
    pub subsets: Vec<Vec<(u32, Colors)>>,
}

pub fn extract_game_info(line: &str) -> GameInfo {
    let mut parts = line.split(':');
    let game_id_str = parts.next().unwrap();
    let game_info = parts.next().unwrap();

    let game_id = game_id_str
        .chars()
        .rev()
        .take_while(|&c| c.is_ascii_digit())
        .collect::<String>()
        .chars()
        .rev()
        .collect::<String>()
        .parse::<u32>()
        .unwrap();

    let subsets: Vec<Vec<(u32, Colors)>> = game_info
        .split(';')
        .map(|s| {
            s.split(',')
                .map(|pair| {
                    let mut parts = pair.split_whitespace();
                    let number: u32 = parts.next().unwrap().parse::<u32>().unwrap();
                    let color = match parts.next().unwrap() {
                        "red" => Colors::R,
                        "green" => Colors::G,
                        "blue" => Colors::B,
                        _ => panic!(),
                    };
                    (number, color)
                })
                .collect()
        })
        .collect();

    GameInfo { game_id, subsets }
}
