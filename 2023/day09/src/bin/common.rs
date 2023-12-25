pub fn get_differences(compute: &mut [Vec<isize>]) -> Vec<isize> {
    let differences: Vec<isize> = compute
        .last()
        .unwrap()
        .windows(2)
        .map(|pair| pair[1] - pair[0])
        .collect();
    differences
}

pub fn get_init_numbers(line: &str) -> Vec<isize> {
    let numbers = line
        .split_whitespace()
        .map(|c| c.parse::<isize>().unwrap())
        .collect::<Vec<isize>>();
    numbers
}
