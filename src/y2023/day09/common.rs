pub fn get_compute(line: &str) -> Vec<Vec<isize>> {
    let numbers = get_init_numbers(line);
    let mut compute = vec![numbers];
    while !compute.last().unwrap().iter().all(|n| n == &0) {
        let differences = get_differences(&mut compute);
        compute.push(differences);
    }
    compute
}

fn get_differences(compute: &mut [Vec<isize>]) -> Vec<isize> {
    compute
        .last()
        .unwrap()
        .windows(2)
        .map(|pair| pair[1] - pair[0])
        .collect()
}

fn get_init_numbers(line: &str) -> Vec<isize> {
    line.split_whitespace()
        .map(|c| c.parse::<isize>().unwrap())
        .collect::<Vec<isize>>()
}
