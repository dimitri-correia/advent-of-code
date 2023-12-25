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

    let mut distances = vec![];
    for (i, &(x, y)) in points.iter().enumerate() {
        for &(xx, yy) in points.iter().skip(i + 1) {
            let dist =
                ((xx as isize - x as isize).abs() + (yy as isize - y as isize).abs()) as usize;
            let from = Pos { x, y };
            let to = Pos { x: xx, y: yy };
            distances.push(Dist { from, to, dist })
        }
    }

    distances
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

    let mut shift = vec![];
    for i in 0..input.lines().next().unwrap().len() {
        if input
            .lines()
            .all(|line| line.chars().nth(i).unwrap() == '.')
        {
            shift.push(i);
        }
    }

    let points: Vec<(usize, usize)> = points
        .iter()
        .map(|(a, b)| {
            (
                *a,
                *b + expansion * shift.iter().filter(|&&i| i < *b).count(),
            )
        })
        .collect();

    points
}
