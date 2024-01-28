use crate::y2023::day09::common;

fn part_1(input: &str) -> String {
    let mut r = 0;
    for line in input.lines() {
        let numbers = common::get_init_numbers(line);
        let mut compute = vec![numbers];
        while !compute.last().unwrap().iter().all(|n| n == &0) {
            let differences = common::get_differences(&mut compute);
            compute.push(differences);
        }

        r += compute.iter().map(|row| row.last().unwrap()).sum::<isize>();
    }
    r.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn actual_challenge() {
        let input = include_str!("input1.txt");
        let output = part_1(input);
        dbg!(&output);
        assert_eq!("1696140818", output);
    }

    #[test]
    fn example_test() {
        let input = include_str!("input1_ex.txt"); // same file
        let r = part_1(input);
        assert_eq!("114", r);
    }
}
