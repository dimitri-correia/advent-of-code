fn part_1(input: &str) -> String {
    let dig_moves = parse_input(input);

    let boarders = get_boarders(dig_moves);

    dbg!(&boarders);

    "".to_string()
}

fn get_boarders(dig_moves: Vec<Instruction>) -> Vec<(i32, i32)> {
    let mut pos = (0, 0);

    let mut boarders = vec![pos];

    for dig_move in dig_moves {
        for _ in 0..dig_move.value {
            match dig_move.direction {
                Direction::U => pos.1 += 1,
                Direction::D => pos.1 -= 1,
                Direction::L => pos.0 -= 1,
                Direction::R => pos.0 += 1,
            }

            boarders.push(pos)
        }
    }
    boarders
}

fn parse_input(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|l| {
            let mut parts = l.split_whitespace();
            let direction = str_to_dir(parts.next().unwrap());
            let value = parts.next().unwrap().parse::<u32>().unwrap();
            let color = parts.next().unwrap().to_string();
            Instruction {
                direction,
                value,
                color,
            }
        })
        .collect()
}

#[derive(Debug)]
struct Instruction {
    direction: Direction,
    value: u32,
    color: String,
}

#[derive(Debug)]
enum Direction {
    R,
    D,
    L,
    U,
}

fn str_to_dir(s: &str) -> Direction {
    match s {
        "R" => Direction::R,
        "D" => Direction::D,
        "L" => Direction::L,
        "U" => Direction::U,
        _ => panic!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn actual_challenge() {
        let input = include_str!("input1.txt");
        let output = part_1(input);
        dbg!(&output);
        assert_eq!("", output);
    }

    #[test]
    fn example_test() {
        let input = include_str!("input1_ex.txt");
        let r = part_1(input);
        assert_eq!("62", r);
    }
}
