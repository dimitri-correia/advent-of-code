pub fn parse_input(input: &str) -> (Vec<i32>, Vec<i32>) {
    let mut list_one = Vec::new();
    let mut list_two = Vec::new();
    input.trim().lines().for_each(|line| {
        let mut parts = line.split_whitespace();
        let first = parts.next().unwrap();
        let second = parts.next().unwrap();
        list_one.push(first.parse().unwrap());
        list_two.push(second.parse().unwrap());
    });

    (list_one, list_two)
}
