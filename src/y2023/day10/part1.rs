use crate::y2023::day10::common::{first_pipe, get_next, read_input};

fn part_1(input: &str) -> String {
    let (lines_vec, start_pos) = read_input(input);

    let mut pass = vec![start_pos];
    let mut pos = first_pipe(&start_pos, &lines_vec);

    loop {
        pass.push(pos);
        let opt_pos = get_next(&pos, &pass, &lines_vec);

        if opt_pos.is_none() {
            break;
        }

        pos = opt_pos.unwrap();
    }

    (pass.len() / 2).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn actual_challenge() {
        let input = include_str!("input1.txt");
        let output = part_1(input);
        dbg!(&output);
        assert_eq!("7145", output);
    }

    #[test]
    fn example_test_a() {
        let input = include_str!("input1_ex.txt");
        let r = part_1(input);
        assert_eq!("4", r);
    }

    #[test]
    fn example_test_b() {
        let input = include_str!("input1_ex2.txt");
        let r = part_1(input);
        assert_eq!("4", r);
    }

    #[test]
    fn example_test_c() {
        let input = include_str!("input1_ex3.txt");
        let r = part_1(input);
        assert_eq!("8", r);
    }

    #[test]
    fn example_test_d() {
        let input = include_str!("input1_ex4.txt");
        let r = part_1(input);
        assert_eq!("8", r);
    }
}
