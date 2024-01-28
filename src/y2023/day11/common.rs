#[derive(Debug)]
pub struct Dist {
    from: Pos,
    to: Pos,
    pub dist: usize,
}

#[derive(Debug)]
pub struct Pos {
    x: usize,
    y: usize,
}

pub fn calculate_distances(input: &str, expansion: usize) -> Vec<Dist> {
    let points = get_points(input, expansion);

    points
        .iter()
        .enumerate()
        .flat_map(|(i, &(x, y))| {
            points.iter().skip(i + 1).map(move |&(xx, yy)| {
                let dist =
                    ((xx as isize - x as isize).abs() + (yy as isize - y as isize).abs()) as usize;
                let from = Pos { x, y };
                let to = Pos { x: xx, y: yy };
                Dist { from, to, dist }
            })
        })
        .collect()
}

fn get_points(input: &str, expansion: usize) -> Vec<(usize, usize)> {
    let mut shift = 0;
    let points: Vec<(usize, usize)> = input
        .lines()
        .enumerate()
        .flat_map(|(x, line)| {
            if !line.contains('#') {
                shift += expansion;
            }
            line.chars()
                .enumerate()
                .filter_map(move |(y, c)| if c == '#' { Some((x + shift, y)) } else { None })
        })
        .collect();

    let shift: Vec<usize> = (0..input.lines().next().unwrap().len())
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
                *b + expansion * shift.iter().filter(|&&i| i < *b).count(),
            )
        })
        .collect()
}
