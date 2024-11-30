pub fn get_number_possibilities(input: &str, total_amount: u32) -> Vec<usize> {
    let mut containers: Vec<u32> = input.lines().map(|line| line.parse().unwrap()).collect();
    containers.sort_unstable_by(|a, b| b.cmp(a));
    // Sort in descending order
    let mut possibilities = vec![];

    let mut stack = vec![(0, 0, 0)];
    while let Some((current_sum, start_index, previous)) = stack.pop() {
        for i in start_index..containers.len() {
            let new_sum = current_sum + containers[i];
            if new_sum == total_amount {
                possibilities.push(previous + 1);
            } else if new_sum < total_amount {
                stack.push((new_sum, i + 1, previous + 1));
            }
        }
    }
    possibilities
}
