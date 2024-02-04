fn part_1(input: &str) -> String {
    input
        .split("\n\n")
        .into_iter()
        .map(|p| get_res_one_pattern(p))
        .sum::<usize>()
        .to_string()
}

fn get_res_one_pattern(input: &str) -> usize {
    match try_find_horizontal_fold(&input) {
        Some(res) => 100 * res,
        _ => try_find_vertical_fold(&input),
    }
}

fn try_find_vertical_fold(input: &&str) -> usize {
    todo!()
}

fn try_find_horizontal_fold(input: &&str) -> Option<usize> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn actual_challenge() {
        let input = include_str!("input1.txt");
        let output = part_1(input);
        dbg!(&output);
        assert_eq!("val", output);
    }

    #[test]
    fn example_test() {
        let input = include_str!("input1_ex.txt"); // same file
        let r = part_1(input);
        assert_eq!("405", r);
    }
}
