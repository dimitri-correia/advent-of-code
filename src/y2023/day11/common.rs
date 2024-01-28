pub fn calculate_distances(input: &str, expansion: usize) -> Vec<usize> {
    let points = get_points(input, expansion);

    points
        .iter()
        .enumerate()
        .flat_map(|(i, &(x, y))| {
            points.iter().skip(i + 1).map(move |&(xx, yy)| {
                dbg!(((x, y), (xx, yy)));
                ((xx as isize - x as isize).abs() + (yy as isize - y as isize).abs()) as usize
            })
        })
        .collect()
}

fn get_points(input: &str, expansion: usize) -> Vec<(usize, usize)> {
    let mut vertical_shift = 0;
    let points: Vec<(usize, usize)> = input
        .lines()
        .enumerate()
        .flat_map(|(x, line)| {
            if !line.contains('#') {
                vertical_shift += expansion;
            }
            line.chars().enumerate().filter_map(move |(y, c)| {
                if c == '#' {
                    Some((x + vertical_shift, y))
                } else {
                    None
                }
            })
        })
        .collect();

    let horizontal_shift: Vec<usize> = (0..input.lines().next().unwrap().len())
        .filter(|&i| {
            input
                .lines()
                .all(|line| line.chars().nth(i).unwrap() == '.')
        })
        .collect();

    points
        .iter()
        .map(|(a, b)| {
            (
                *a,
                *b + expansion * horizontal_shift.iter().filter(|&&i| i < *b).count(),
            )
        })
        .collect()
}
