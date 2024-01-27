fn part_2(input: &str) -> String {
    let (time, distance_to_beat) = read_input(input);
    compute_number_of_ways_to_win(time, distance_to_beat).to_string()
}

fn compute_number_of_ways_to_win(total_time: usize, distance_to_beat: usize) -> i32 {
    let mut res = 0;

    for charging_time in 0..=total_time {
        let speed = charging_time;
        let time_of_travel = total_time - charging_time;
        let dist = speed * time_of_travel;

        if dist > distance_to_beat {
            res += 1;
        }
    }

    res
}

fn read_input(input: &str) -> (usize, usize) {
    let mut lines = input.lines();
    let time: usize = lines
        .next()
        .unwrap()
        .chars()
        .filter(|c| c.is_numeric())
        .collect::<String>()
        .parse::<usize>()
        .unwrap();
    let distance_to_beat: usize = lines
        .next()
        .unwrap()
        .chars()
        .filter(|c| c.is_numeric())
        .collect::<String>()
        .parse::<usize>()
        .unwrap();

    (time, distance_to_beat)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn actual_challenge() {
        let input = include_str!("input1.txt");
        let output = part_2(input);
        dbg!(&output);
        assert_eq!("60568880", output);
    }

    #[test]
    fn example_test() {
        let input = include_str!("input1_ex.txt"); // same file
        let r = part_2(input);
        assert_eq!("71503", r);
    }
}
