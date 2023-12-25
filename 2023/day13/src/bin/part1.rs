fn main() {
    let input = include_str!("./input1.txt");
    let output = part_1(input);
    dbg!(output);
}

fn part_1(input: &str) -> String {
    let mut r = 0;
    for p in input.split("\n\n") {
        r += get_res_one_pattern(p);
    }

    r.to_string()
}

#[derive(Debug)]
struct Reflexion {
    from: usize,
    to: usize,
    orientation: Orientation,
}

#[derive(Debug)]
enum Orientation {
    Horizontal,
    Vertical,
}

fn get_res_one_pattern(input: &str) -> usize {
    let mut rows = Vec::with_capacity(input.lines().count());
    let mut cols: Vec<String> = vec!["".to_string(); input.lines().next().unwrap().len()];

    for line in input.lines() {
        rows.push(line);
        for (idx, c) in line.chars().enumerate() {
            cols[idx] += &c.to_string();
        }
    }

    let mut same_rows = vec![];
    for (idx, r) in rows.iter().enumerate() {
        for (idx2, rr) in rows.iter().skip(idx + 1).enumerate() {
            if &r == &rr {
                same_rows.push((idx, idx2 + idx + 1));
            }
        }
    }

    let mut same_cols = vec![];
    for (idx, c) in cols.iter().enumerate() {
        for (idx2, cc) in cols.iter().skip(idx + 1).enumerate() {
            if &c == &cc {
                same_cols.push((idx, idx2 + idx + 1));
            }
        }
    }

    let mut max_reflexion = Reflexion {
        from: 0,
        to: 0,
        orientation: Orientation::Vertical,
    };
    for (mut a, mut b) in same_cols.iter() {
        if a + 1 != b {
            continue;
        }
        for (i, j) in same_cols.iter().rev() {
            if (i + 1 == a) && (j == &(b + 1)) {
                a = *i;
                b = *j;
            } else if i + 2 <= a {
                break;
            }
        }
        if b - a > max_reflexion.to - max_reflexion.from {
            max_reflexion = Reflexion {
                from: a + 1,
                to: b + 1,
                orientation: Orientation::Vertical,
            };
        }
    }
    for (mut a, mut b) in same_rows.iter() {
        if a + 1 != b {
            continue;
        }
        for (i, j) in same_rows.iter().rev() {
            if (i + 1 == a) && (j == &(b + 1)) {
                a = *i;
                b = *j;
            } else if i + 2 <= a {
                break;
            }
        }
        if b - a > max_reflexion.to - max_reflexion.from {
            max_reflexion = Reflexion {
                from: a + 1,
                to: b + 1,
                orientation: Orientation::Horizontal,
            };
        }
    }

    let n = (max_reflexion.from + max_reflexion.to) / 2;

    match &max_reflexion.orientation {
        Orientation::Vertical => n,
        Orientation::Horizontal => 100 * n,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("./input1_ex.txt");
        let r = part_1(input);
        assert_eq!("405", r);
    }
}
