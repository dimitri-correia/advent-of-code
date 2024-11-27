use super::common::{compute_dests_and_cities, get_all_routes_length};

pub fn part_1_example() -> String {
    let input = include_str!("input1_ex.txt");
    part_1(input)
}

pub fn part_1_actual_challenge() -> String {
    let input = include_str!("input1.txt");
    part_1(input)
}

fn part_1(input: &str) -> String {
    let (dists, cities) = compute_dests_and_cities(input);

    let nb_cities = cities.len();

    get_all_routes_length(cities, nb_cities, dists)
        .min()
        .unwrap()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn actual_challenge() {
        let output = part_1_actual_challenge();        
        assert_eq!("141", output);
    }

    #[test]
    fn example_test() {
        let r = part_1_example();
        assert_eq!("605", r);
    }
}
