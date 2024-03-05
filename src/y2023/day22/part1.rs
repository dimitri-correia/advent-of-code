fn part_1(input: &str) -> String {
    let bricks = parse_input(input);
    dbg!(&bricks);
    "".to_string()
}

fn parse_input(input: &str) -> Vec<Brick> {
    input
        .lines()
        .map(|l| {
            let mut parts = l.split('~');
            Brick {
                cornerA: parse_pos(parts.next().unwrap()),
                cornerB: parse_pos(parts.next().unwrap()),
            }
        })
        .collect()
}

fn parse_pos(pos: &str) -> Pos {
    let mut parts = pos.split(',');
    Pos {
        x: parts.next().unwrap().parse().unwrap(),
        y: parts.next().unwrap().parse().unwrap(),
        z: parts.next().unwrap().parse().unwrap(),
    }
}

#[derive(Debug)]
struct Brick {
    cornerA: Pos,
    cornerB: Pos,
}

#[derive(Debug)]
struct Pos {
    x: isize,
    y: isize,
    z: isize,
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
        assert_eq!("5", r);
    }
}
