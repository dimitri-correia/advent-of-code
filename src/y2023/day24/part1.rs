use itertools::Itertools;

fn part_1(input: &str, time_frame: IntersectionBetween) -> String {
    let hails = parse_input(input);

    dbg!(&hails);

    let hails_move_in_time_frame = advance_hails_of(&hails, &time_frame);

    dbg!(&hails_move_in_time_frame);

    count_intersections(hails_move_in_time_frame).to_string()
}

fn count_intersections(hail: Vec<HailLine>) -> usize {
    hail.iter()
        .combinations(2)
        .filter(|pair| intersect_exists_between(pair[0], pair[1]))
        .count()
}

fn intersect_exists_between(line1: &HailLine, line2: &HailLine) -> bool {
    todo!()
}

fn advance_hails_of(hails_before: &Vec<Hail>, time: &IntersectionBetween) -> Vec<HailLine> {
    hails_before
        .iter()
        .map(|hail| {
            let start = Pos {
                px: hail.pos.px + time.start * hail.vel.vx,
                py: hail.pos.py + time.start * hail.vel.vy,
                pz: hail.pos.pz + time.start * hail.vel.vz,
            };
            let end = Pos {
                px: start.px + time.end * hail.vel.vx,
                py: start.py + time.end * hail.vel.vy,
                pz: start.pz + time.end * hail.vel.vz,
            };
            HailLine { start, end }
        })
        .collect()
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
struct HailLine {
    start: Pos,
    end: Pos,
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
