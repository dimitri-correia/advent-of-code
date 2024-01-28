use crate::y2023::day10::common::Pipe::{
    NorthEast, NorthWest, SouthEast, SouthWest, StartingPosition, Vertical,
};
use crate::y2023::day10::common::{get_path, Pipe};

fn part_2(input: &str) -> String {
    let (pass, lines_vec) = get_path(input);

    count_point_inside(&pass, lines_vec).to_string()
}

fn count_point_inside(pass: &[(usize, usize)], line_vec: Vec<Vec<Pipe>>) -> usize {
    line_vec
        .iter()
        .enumerate()
        .map(|(x, line)| count_line(pass, &x, line))
        .sum()
}

fn count_line(pass: &[(usize, usize)], x: &usize, line: &Vec<Pipe>) -> usize {
    let mut inside_loop = false;

    let mut res = 0;

    for (y, pipe) in line.iter().enumerate() {
        if pass.contains(&(*x, y)) {
            if change_in_out(pipe, line, y) {
                inside_loop = !inside_loop;
            }
        } else if inside_loop {
            res += 1;
        }
    }

    res
}

fn change_in_out(pipe: &Pipe, line: &Vec<Pipe>, y: usize) -> bool {
    let change_in_out = [Vertical, NorthEast, NorthWest];
    let change_in_out_if_start = [Vertical, SouthWest, SouthEast];

    change_in_out.contains(pipe)
        || (pipe == &StartingPosition
            && (y != 0 && change_in_out_if_start.contains(line.get(y - 1).unwrap())))
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
