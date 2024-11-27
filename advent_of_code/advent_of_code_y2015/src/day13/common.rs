use itertools::Itertools;
use std::collections::HashMap;

pub fn compute_best_happiness(all_names: Vec<&str>, arrangements: HashMap<String, i32>) -> String {
    let len = all_names.len();
    all_names
        .into_iter()
        .permutations(len)
        .map(|permutation| {
            let mut total = 0;
            for i in 0..len {
                let name = permutation[i];
                let next_name = permutation[(i + 1) % len];
                let names = if name > next_name {
                    format!("{}, {}", next_name, name)
                } else {
                    format!("{}, {}", name, next_name)
                };
                total += arrangements.get(&names).unwrap_or(&0);
            }
            total
        })
        .max()
        .unwrap()
        .to_string()
}

pub fn parse_input(input: &str) -> (HashMap<String, i32>, Vec<&str>) {
    //Alice would gain 54 happiness units by sitting next to Bob.
    //Alice would lose 79 happiness units by sitting next to Carol.
    let mut arrangements: HashMap<String, i32> = HashMap::new();
    let mut all_names: Vec<&str> = vec![];
    input.lines().for_each(|line| {
        let parts = line.trim().split_whitespace().collect::<Vec<&str>>();
        let name = parts[0];
        if all_names.iter().find(|&&x| x == name).is_none() {
            all_names.push(name);
        }
        let sign = parts[2];
        let value = parts[3].parse::<i32>().unwrap();
        let other_name = parts[10].trim_end_matches('.');
        let mut value = if sign == "gain" { value } else { -value };
        let names = if name > other_name {
            format!("{}, {}", other_name, name)
        } else {
            format!("{}, {}", name, other_name)
        };
        value += arrangements.get(&names).unwrap_or(&0);
        arrangements.insert(names, value);
    });
    (arrangements, all_names)
}
