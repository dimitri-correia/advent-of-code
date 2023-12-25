fn main() {
    let input = include_str!("./input1.txt");
    let output = part_1(input);
    dbg!(output);
}

fn part_1(input: &str) -> String {
    let (times, distances_to_beat): (Vec<usize>, Vec<usize>) = read_input(input);
    dbg!((&times, &distances_to_beat));
    let res = compute_number_of_ways_to_win(times, distances_to_beat);
    res.to_string()
}

fn read_input(input: &str) -> (Vec<usize>, Vec<usize>) {
    let mut times: Vec<usize> = vec![];
    let mut distances_to_beat: Vec<usize> = vec![];

    let mut lines = input.lines();
    let parts = lines
        .next()
        .unwrap()
        .trim()
        .split(':')
        .last()
        .unwrap()
        .split_whitespace();
    for i in parts {
        times.push(i.parse::<usize>().unwrap());
    }
    let parts = lines
        .next()
        .unwrap()
        .trim()
        .split(':')
        .last()
        .unwrap()
        .split_whitespace();
    for i in parts {
        distances_to_beat.push(i.parse::<usize>().unwrap());
    }
    (times, distances_to_beat)
}

fn compute_number_of_ways_to_win(times: Vec<usize>, distances_to_beat: Vec<usize>) -> i32 {
    let mut global_res = 1;

    for (idx, &total_time) in times.iter().enumerate() {
        let mut local_res = 0;

        for charging_time in 0..=total_time {
            let speed = charging_time;
            let time_of_travel = total_time - charging_time;
            let dist = speed * time_of_travel;

            if dist > distances_to_beat[idx] {
                local_res += 1;
            }
        }

        global_res *= local_res;
    }

    global_res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("./input1_ex.txt");
        let r = part_1(input);
        assert_eq!("288", r);
    }
}
