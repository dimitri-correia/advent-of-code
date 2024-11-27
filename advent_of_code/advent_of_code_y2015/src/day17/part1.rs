pub fn part_1_example() -> String {
    let input = include_str!("input1_ex.txt");
    let total_amount = 25;
    part_1(input, total_amount)
}

pub fn part_1_actual_challenge() -> String {
    let input = include_str!("input1.txt");
    let total_amount = 150;
    part_1(input, total_amount)
}

fn part_1(input: &str, total_amount: u32) -> String {
    let mut containers: Vec<u32> = input.lines().map(|line| line.parse().unwrap()).collect();
    containers.sort_unstable_by(|a, b| b.cmp(a)); // Sort in descending order
    let mut nb_possibilities = 0;

    let mut stack = vec![(0, 0)];
    while let Some((current_sum, start_index)) = stack.pop() {
        for i in start_index..containers.len() {
            let new_sum = current_sum + containers[i];
            if new_sum == total_amount {
                nb_possibilities += 1;
            } else if new_sum < total_amount {
                stack.push((new_sum, i + 1));
            }
        }
    }

    nb_possibilities.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn actual_challenge() {
        let output = part_1_actual_challenge();
        assert_eq!("1638", output);
    }

    #[test]
    fn example_test() {
        let r = part_1_example();
        assert_eq!("4", r);
    }
}
