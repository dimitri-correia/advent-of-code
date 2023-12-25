#[derive(PartialEq, Eq, Hash, Debug)]
pub enum Colors {
    R,
    G,
    B,
}

pub fn extract_game_info(line: &str) -> (u32, Vec<Vec<(u32, Colors)>>) {
    let mut parts = line.split(':');
    let game_id_str = parts.next().expect("Invalid input: missing game ID");
    let game_info = parts.next().expect("Invalid input: missing game info");

    let game_id = game_id_str
        .chars()
        .rev()
        .take_while(|&c| c.is_ascii_digit())
        .collect::<String>()
        .chars()
        .rev()
        .collect::<String>()
        .parse()
        .unwrap_or_else(|_| panic!("Failed to parse game ID: {}", game_id_str));

    let subsets: Vec<Vec<(u32, Colors)>> = game_info
        .split(';')
        .map(|s| {
            s.split(',')
                .map(|pair| {
                    let mut parts = pair.split_whitespace();
                    let number: u32 = parts
                        .next()
                        .expect("Missing number")
                        .parse()
                        .expect("Failed to parse number");
                    let color = match parts.next().expect("Missing color") {
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

    (game_id, subsets)
}
