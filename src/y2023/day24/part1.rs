fn part_1(input: &str, intersection: IntersectionBetween) -> String {
    let hails = parse_input(input);

    dbg!(&hails);

    "".to_string()
}

fn parse_input(input: &str) -> Vec<Hail> {
    input
        .lines()
        .map(|l| {
            let mut parts = l.split('@');
            Hail {
                pos: parse_pos(parts.next().unwrap()),
                vel: parse_vel(parts.next().unwrap()),
            }
        })
        .collect()
}

fn parse_vel(vel: &str) -> Velocity {
    let mut parts = vel.split(',');
    Velocity {
        vx: parts.next().unwrap().trim().parse().unwrap(),
        vy: parts.next().unwrap().trim().parse().unwrap(),
        vz: parts.next().unwrap().trim().parse().unwrap(),
    }
}

fn parse_pos(pos: &str) -> Pos {
    let mut parts = pos.split(',');
    Pos {
        px: parts.next().unwrap().trim().parse().unwrap(),
        py: parts.next().unwrap().trim().parse().unwrap(),
        pz: parts.next().unwrap().trim().parse().unwrap(),
    }
}

#[derive(Debug)]
struct Hail {
    pos: Pos,
    vel: Velocity,
}

#[derive(Debug)]
struct Pos {
    px: isize,
    py: isize,
    pz: isize,
}

#[derive(Debug)]
struct Velocity {
    vx: isize,
    vy: isize,
    vz: isize,
}

#[derive(Debug)]
struct IntersectionBetween {
    start: isize,
    end: isize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn actual_challenge() {
        let input = include_str!("input1.txt");
        let output = part_1(
            input,
            IntersectionBetween {
                start: 200_000_000_000_000,
                end: 400_000_000_000_000,
            },
        );
        dbg!(&output);
        assert_eq!("", output);
    }

    #[test]
    fn example_test() {
        let input = include_str!("input1_ex.txt");
        let r = part_1(input, IntersectionBetween { start: 7, end: 27 });
        assert_eq!("", r);
    }
}
