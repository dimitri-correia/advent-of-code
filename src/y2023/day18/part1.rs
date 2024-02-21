use itertools::Itertools;

fn part_1(input: &str) -> String {
    let dig_moves = parse_input(input);

    let boarders: Vec<(i32, i32)> = get_boarders(dig_moves);

    get_all_positions_inside(&boarders).to_string()
}

fn get_all_positions_inside(boarders: &[(i32, i32)]) -> i32 {
    let mut x_ordered = boarders.iter().sorted_by(|(a, _), (b, _)| a.cmp(b));
    let x_min = x_ordered.next().unwrap().0;
    let x_max = x_ordered.last().unwrap().0;

    let mut y_ordered = boarders.iter().sorted_by(|(_, a), (_, b)| a.cmp(b));
    let y_min = y_ordered.next().unwrap().1;
    let y_max = y_ordered.last().unwrap().1;

    (y_min..=y_max)
        .map(|y| dbg!(compute_sum_for_line(boarders, x_min, x_max, y)))
        .sum()
}

fn compute_sum_for_line(boarders: &[(i32, i32)], x_min: i32, x_max: i32, y: i32) -> i32 {
    (x_min..=x_max)
        .into_iter()
        .fold((0, false, false), |(s, inside, prev_was_boarder), x| {
            let is_border = boarders.contains(&(x, y));

            if !is_border {
                return if inside {
                    (s + 1, inside, is_border)
                } else {
                    (s, inside, is_border)
                };
            }

            //is border true
            if inside == prev_was_boarder {
                return (s + 1, !inside, is_border);
            }
            if inside {
                return (s + 1, false, is_border);
            }
            return (s + 1, true, is_border);
        })
        .0
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

    #[test]
    fn test_one_line() {
        // .###.#...
        let boarders = [(1, 0), (2, 0), (3, 0), (5, 0)];
        let computed = compute_sum_for_line(&boarders, 0, 8, 0);
        let expected = 5;
        assert_eq!(expected, computed);
    }
}
