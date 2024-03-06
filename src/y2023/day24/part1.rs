use itertools::Itertools;

fn part_1(input: &str, time_frame: IntersectionBetween) -> String {
    let hails = parse_input(input);

    let hails_move_in_time_frame = advance_hails_of(&hails, &time_frame);

    count_intersections(hails_move_in_time_frame).to_string()
}

fn count_intersections(hail: Vec<HailLine>) -> usize {
    hail.iter()
        .combinations(2)
        .filter(|pair| intersect_exists_between(pair[0], pair[1]))
        .count()
}

fn intersect_exists_between(line1: &HailLine, line2: &HailLine) -> bool {
    // https://stackoverflow.com/questions/563198/how-do-you-detect-where-two-line-segments-intersect/

    let p0_x = line1.start.px;
    let p0_y = line1.start.py;

    let p1_x = line1.end.px;
    let p1_y = line1.end.py;

    let p2_x = line2.start.px;
    let p2_y = line2.start.py;

    let p3_x = line2.end.px;
    let p3_y = line2.end.py;

    let s10_x = p1_x - p0_x;
    let s10_y = p1_y - p0_y;

    let s32_x = p3_x - p2_x;
    let s32_y = p3_y - p2_y;

    let denom = s10_x * s32_y - s32_x * s10_y;
    if denom == 0 {
        return false;
    }
    let denom_positive = denom > 0;

    let s02_x = p0_x - p2_x;
    let s02_y = p0_y - p2_y;
    let s_numer = s10_x * s02_y - s10_y * s02_x;
    if (s_numer < 0) == denom_positive {
        return false;
    }

    let t_numer = s32_x * s02_y - s32_y * s02_x;
    if (t_numer < 0) == denom_positive {
        return false;
    }

    if (s_numer > denom) == denom_positive || (t_numer > denom) == denom_positive {
        return false;
    }
    // Collision detected
    // let t = t_numer / denom;
    // dbg!((p0_x + t * s10_x), (p0_y + t * s10_y));

    true
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
    px: i64,
    py: i64,
    pz: i64,
}

#[derive(Debug)]
struct Velocity {
    vx: i64,
    vy: i64,
    vz: i64,
}

#[derive(Debug)]
struct IntersectionBetween {
    start: i64,
    end: i64,
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
        assert_eq!("2", r);
    }

    #[test]
    fn test_line_intersection() {
        assert!(intersect_exists_between(
            &HailLine {
                start: Pos {
                    px: 0,
                    py: 0,
                    pz: 0
                },
                end: Pos {
                    px: 2,
                    py: 0,
                    pz: 0
                }
            },
            &HailLine {
                start: Pos {
                    px: 1,
                    py: 0,
                    pz: 0
                },
                end: Pos {
                    px: 1,
                    py: 1,
                    pz: 0
                }
            }
        ));
        assert!(intersect_exists_between(
            &HailLine {
                start: Pos {
                    px: 0,
                    py: 0,
                    pz: 0
                },
                end: Pos {
                    px: 3,
                    py: 3,
                    pz: 0
                }
            },
            &HailLine {
                start: Pos {
                    px: 0,
                    py: 3,
                    pz: 0
                },
                end: Pos {
                    px: 3,
                    py: 0,
                    pz: 0
                }
            }
        ));
        assert!(!intersect_exists_between(
            &HailLine {
                start: Pos {
                    px: 0,
                    py: 0,
                    pz: 0
                },
                end: Pos {
                    px: 1,
                    py: 0,
                    pz: 0
                }
            },
            &HailLine {
                start: Pos {
                    px: 2,
                    py: 0,
                    pz: 0
                },
                end: Pos {
                    px: 0,
                    py: 0,
                    pz: 0
                }
            }
        ));
        assert!(!intersect_exists_between(
            &HailLine {
                start: Pos {
                    px: 0,
                    py: 0,
                    pz: 0
                },
                end: Pos {
                    px: 1,
                    py: 0,
                    pz: 0
                }
            },
            &HailLine {
                start: Pos {
                    px: 0,
                    py: 1,
                    pz: 0
                },
                end: Pos {
                    px: 1,
                    py: 1,
                    pz: 0
                }
            }
        ));
        assert!(!intersect_exists_between(
            &HailLine {
                start: Pos {
                    px: 0,
                    py: 0,
                    pz: 0
                },
                end: Pos {
                    px: 0,
                    py: 1,
                    pz: 0
                }
            },
            &HailLine {
                start: Pos {
                    px: 1,
                    py: 0,
                    pz: 0
                },
                end: Pos {
                    px: 1,
                    py: 1,
                    pz: 0
                }
            }
        ));
    }
}
