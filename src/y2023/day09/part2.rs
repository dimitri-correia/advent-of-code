use crate::y2023::day09::common;

fn part_2(input: &str) -> String {
    let mut r = 0;
    for line in input.lines() {
        let numbers = common::get_init_numbers(line);
        let mut compute = vec![numbers];
        while !compute.last().unwrap().iter().all(|n| n == &0) {
            let differences = common::get_differences(&mut compute);
            compute.push(differences);
        }

        r += compute
            .iter()
            .rev()
            .fold(0, |acc, l| l.first().map_or(acc, |&first| first - acc));
    }
    r.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn actual_challenge() {
        let input = include_str!("input1.txt");
        let output = part_2(input);
        dbg!(&output);
        assert_eq!("1152", output);
    }

    #[test]
    fn example_test() {
        let input = include_str!("input1_ex.txt"); // same file
        let r = part_2(input);
        assert_eq!("2", r);
    }
}
