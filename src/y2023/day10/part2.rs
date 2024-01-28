use crate::y2023::day10::common::Pipe::{SouthEast, SouthWest, StartingPosition, Vertical};
use crate::y2023::day10::common::{get_path, Pipe};

fn part_2(input: &str) -> String {
    let (pass, lines_vec) = get_path(input);

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn actual_challenge() {
        let input = include_str!("input1.txt");
        let output = part_2(input);
        dbg!(&output);
        assert_eq!("445", output);
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
