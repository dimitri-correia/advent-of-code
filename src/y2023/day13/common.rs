pub fn solve(input: &str, find_fold: fn(Vec<&str>) -> Option<usize>) -> String {
    input
        .split("\n\n")
        .into_iter()
        .map(|p| get_res_one_pattern(p, find_fold))
        .sum::<usize>()
        .to_string()
}

fn get_res_one_pattern(input: &str, find_fold: fn(Vec<&str>) -> Option<usize>) -> usize {
    match try_find_horizontal_fold(input, find_fold) {
        Some(res) => 100 * res,
        _ => try_find_vertical_fold(&input, find_fold).unwrap(),
    }
}

fn try_find_vertical_fold(
    lines_input: &str,
    find_fold: fn(Vec<&str>) -> Option<usize>,
) -> Option<usize> {
    let lines: Vec<&str> = lines_input.lines().collect();

    let col: Vec<String> = (0..lines[0].len())
        .map(|i| lines.iter().map(|s| s[i..=i].to_owned()).collect())
        .collect();

    find_fold(col.iter().map(|c| c.as_str()).collect())
}

fn try_find_horizontal_fold(
    lines_input: &str,
    find_fold: fn(Vec<&str>) -> Option<usize>,
) -> Option<usize> {
    find_fold(lines_input.lines().collect())
}
