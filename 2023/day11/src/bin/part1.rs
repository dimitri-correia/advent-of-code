fn main() {
    let input = include_str!("./input1.txt");
    let output = part_1(input);
    dbg!(output);
}

fn part_1(input: &str) -> String {
    let distances = calculate_distances(input);

    let sum: usize = distances.iter().map(|d| d.dist).sum();

    sum.to_string()
}

#[derive(Debug)]
struct Dist {
    from: Pos,
    to: Pos,
    dist: usize,
}

#[derive(Debug)]
struct Pos {
    x: usize,
    y: usize,
}

fn calculate_distances(input: &str) -> Vec<Dist> {
    let points = get_points(input);

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

fn get_points(input: &str) -> Vec<(usize, usize)> {
    let mut shift = 0;
    let points: Vec<(usize, usize)> = input
        .lines()
        .enumerate()
        .flat_map(|(x, line)| {
            if !line.contains('#') {
                shift += 1;
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
        .map(|(a, b)| (*a, *b + shift.iter().filter(|&&i| i < *b).count()))
        .collect();

    points
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("./input1_ex.txt");
        let r = part_1(input);
        assert_eq!("374", r);
    }
}
