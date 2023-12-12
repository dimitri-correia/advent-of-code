mod common;

fn main() {
    let input = include_str!("./input1.txt");
    let output = part_1(input);
    dbg!(output);
}

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
    fn it_works() {
        let input = include_str!("./input1_ex.txt");
        let r = part_1(input);
        assert_eq!("114", r);
    }
}
