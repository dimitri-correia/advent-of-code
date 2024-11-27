use itertools::Itertools;
use std::collections::{HashMap, HashSet};

pub fn get_all_routes_length<'a>(
    cities: HashSet<&'a str>,
    nb_cities: usize,
    dists: HashMap<(&'a str, &'a str), u32>,
) -> std::iter::Map<
    itertools::Permutations<std::collections::hash_set::IntoIter<&'a str>>,
    impl FnMut(Vec<&'a str>) -> u32,
> {
    let dists = dists.clone();
    cities.into_iter().permutations(nb_cities).map(move |p| {
        p.windows(2)
            .map(|w| {
                dists
                    .get(&(w[0], w[1]))
                    .or_else(|| dists.get(&(w[1], w[0])))
                    .unwrap()
            })
            .sum::<u32>()
    })
}

pub fn compute_dests_and_cities(input: &str) -> (HashMap<(&str, &str), u32>, HashSet<&str>) {
    let mut dists = HashMap::new();
    let mut cities = HashSet::new();
    input.lines().for_each(|l| {
        let mut parts = l.split_whitespace();
        let city_a = parts.next().unwrap();
        let city_b = parts.nth(1).unwrap();
        let dist = parts.last().unwrap().parse::<u32>().unwrap();
        dists.insert((city_a, city_b), dist);
        cities.insert(city_a);
        cities.insert(city_b);
    });
    (dists, cities)
}
