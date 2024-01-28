use crate::y2023::day10::common::Pipe::{SouthEast, SouthWest, StartingPosition, Vertical};
use crate::y2023::day10::common::{first_pipe, get_next, read_input, Pipe};

fn part_2(input: &str) -> String {
    let (lines_vec, start_pos) = read_input(input);

    let pass = get_path(&lines_vec, start_pos);

    count_point_inside(&pass, lines_vec).to_string()
}

#[derive(Debug, Eq, PartialEq)]
enum Status {
    In,
    Out,
}

fn count_point_inside(pass: &[(usize, usize)], line_vec: Vec<Vec<Pipe>>) -> usize {
    let change_in_out = [StartingPosition, Vertical, SouthWest, SouthEast];
    line_vec
        .iter()
        .enumerate()
        .map(|(x, l)| {
            let mut status = Status::Out;
            l.iter()
                .enumerate()
                .filter(|(y, pipe)| {
                    if !(pass.contains(&(x, *y))) {
                        match status {
                            Status::In => true,
                            Status::Out => false,
                        }
                    } else {
                        if change_in_out.contains(pipe) {
                            status = match status {
                                Status::In => Status::Out,
                                Status::Out => Status::In,
                            };
                        };
                        false
                    }
                })
                .count()
        })
        .sum::<usize>()
}

fn get_path(lines_vec: &[Vec<Pipe>], start_pos: (usize, usize)) -> Vec<(usize, usize)> {
    let mut pass = vec![start_pos];
    let mut pos = first_pipe(&start_pos, lines_vec);

    loop {
        pass.push(pos);
        let opt_pos = get_next(&pos, &pass, lines_vec);

        if opt_pos.is_none() {
            break;
        }

        pos = opt_pos.unwrap();
    }
    pass
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn actual_challenge() {
        let input = include_str!("input1.txt");
        let output = part_2(input);
        dbg!(&output);
        assert_eq!("454", output);
    }

    #[test]
    fn example_test_a() {
        let input = include_str!("input2_ex1.txt");
        let r = part_2(input);
        assert_eq!("4", r);
    }

    #[test]
    fn example_test_b() {
        let input = include_str!("input2_ex2.txt");
        let r = part_2(input);
        assert_eq!("8", r);
    }

    #[test]
    fn example_test_c() {
        let input = include_str!("input2_ex3.txt");
        let r = part_2(input);
        assert_eq!("10", r);
    }
}
