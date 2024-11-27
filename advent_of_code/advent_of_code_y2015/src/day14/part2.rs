use super::common::parse_input;

pub fn part_2_example() -> String {
    let input = include_str!("input1_ex.txt");
    part_2(input, 1000)
}

pub fn part_2_actual_challenge() -> String {
    let input = include_str!("input1.txt");
    part_2(input, 2503)
}

fn part_2(input: &str, time_in_s: u16) -> String {
    let reindeers = parse_input(input);
    let mut scores = vec![0; reindeers.len()];
    let mut distances = vec![0; reindeers.len()];

    for t in 1..=time_in_s {
        for (i, reindeer) in reindeers.iter().enumerate() {
            distances[i] = reindeer.distance_after(t);
        }
        let max_distance = *distances.iter().max().unwrap();
        for (i, &distance) in distances.iter().enumerate() {
            if distance == max_distance {
                scores[i] += 1;
            }
        }
    }

    scores.iter().max().unwrap().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn actual_challenge() {
        let output = part_2_actual_challenge();assert_eq!("1084", output);
    }
    #[test]
    fn example_test() {
        let r = part_2_example();
        assert_eq!("689", r);
    }
}
