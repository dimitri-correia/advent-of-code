use itertools::Itertools;

fn part_2(input: &str) -> String {
    input
        .split("\n\n")
        .into_iter()
        .map(|p| get_res_one_pattern(p))
        .sum::<usize>()
        .to_string()
}

fn get_res_one_pattern(input: &str) -> usize {
    match try_find_horizontal_fold(input) {
        Some(res) => 100 * res,
        _ => try_find_vertical_fold(&input).unwrap(),
    }
}

fn try_find_vertical_fold(lines_input: &str) -> Option<usize> {
    let lines: Vec<&str> = lines_input.lines().collect();

    let col: Vec<String> = (0..lines[0].len())
        .map(|i| lines.iter().map(|s| s[i..=i].to_owned()).collect())
        .collect();

    find_fold(col.iter().map(|c| c.as_str()).collect())
}

fn try_find_horizontal_fold(lines_input: &str) -> Option<usize> {
    find_fold(lines_input.lines().collect())
}

fn find_fold(lines: Vec<&str>) -> Option<usize> {
    lines
        .iter()
        .enumerate()
        .tuple_windows()
        .filter(|((_, line_a), (_, line_b))| {
            line_a
                .chars()
                .zip(line_b.chars())
                .filter(|(a, b)| a != b)
                .count()
                <= 1
        })
        .find_map(|(_, (index_b, _))| {
            let (before, after) = lines.split_at(index_b);

            let all_lines_equal = 1
                == before
                    .iter()
                    .rev()
                    .join("")
                    .chars()
                    .zip(after.join("").chars())
                    .filter(|(a, b)| a != b)
                    .count();

            all_lines_equal.then_some(index_b)
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn actual_challenge() {
        let input = include_str!("input1.txt");
        let output = part_2(input);
        dbg!(&output);
        assert_eq!("37453", output);
    }

    #[test]
    fn example_test() {
        let input = include_str!("input1_ex.txt"); // same file
        let r = part_2(input);
        assert_eq!("400", r);
    }
}
