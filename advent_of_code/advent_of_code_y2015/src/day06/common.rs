pub enum Instruction {
    TurnOn,
    TurnOff,
    Toggle,
}

impl Instruction {
    pub fn from_str(s: &str, ss: &str) -> (Self, bool) {
        match s {
            "turn" => (
                match ss {
                    "on" => Self::TurnOn,
                    "off" => Self::TurnOff,
                    _ => panic!("Invalid instruction"),
                },
                true,
            ),
            "toggle" => (Self::Toggle, false),
            _ => panic!("Invalid instruction"),
        }
    }
}

pub fn parse_line(s: &str) -> (Instruction, (usize, usize), (usize, usize)) {
    let parts: Vec<&str> = s.split_whitespace().collect();
    let (instruction, have_used_part_1) = Instruction::from_str(parts[0], parts[1]);
    let part_idx = if have_used_part_1 { 2 } else { 1 };
    let start: Vec<usize> = parts[part_idx]
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect();
    let end: Vec<usize> = parts[part_idx + 2]
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect();
    (instruction, (start[0], start[1]), (end[0], end[1]))
}
