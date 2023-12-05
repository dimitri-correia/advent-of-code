use std::cmp::min;
use std::str::Lines;

pub enum Part {
    Part1,
    Part2,
}

#[derive(Debug)]
pub struct MapD {
    destination_range_start: usize,
    source_range_start: usize,
    range_length: usize,
}

#[derive(Debug)]
pub struct Input {
    pub seeds: Vec<usize>,
    seed_to_soil: Vec<MapD>,
    soil_to_fertilizer: Vec<MapD>,
    fertilizer_to_water: Vec<MapD>,
    water_to_light: Vec<MapD>,
    light_to_temperature: Vec<MapD>,
    temperature_to_humidity: Vec<MapD>,
    humidity_to_location: Vec<MapD>,
}

pub fn get_min_location(input: Input) -> usize {
    let mut res = usize::MAX;

    for mut seed in input.seeds {
        dbg!(&seed);
        for map in [
            &input.seed_to_soil,
            &input.soil_to_fertilizer,
            &input.fertilizer_to_water,
            &input.water_to_light,
            &input.light_to_temperature,
            &input.temperature_to_humidity,
            &input.humidity_to_location,
        ]
        .iter()
        {
            //naming to change
            seed = match map.iter().find(|sts| {
                (sts.source_range_start..=sts.source_range_start + sts.range_length).contains(&seed)
            }) {
                Some(m) => seed + m.destination_range_start - m.source_range_start,
                _ => seed,
            };
        }
        dbg!(seed);
        res = min(res, seed);
    }
    res
}

pub fn get_maps(input: &str, part: Part) -> Input {
    let mut lines = input.lines();

    let seeds: Vec<usize> = lines
        .next()
        .unwrap()
        .split(':')
        .last()
        .unwrap()
        .split_whitespace()
        .map(|n| n.parse::<usize>().unwrap())
        .collect();

    let seeds: Vec<usize> = match part {
        Part::Part1 => seeds,
        Part::Part2 => {
            let mut new_seeds = vec![];
            for chunk in seeds.chunks(2) {
                match chunk {
                    &[start, range] => {
                        for s in start..start + range {
                            new_seeds.push(s);
                        }
                    }
                    _ => panic!(),
                }
            }
            new_seeds
        }
    };

    let mut seed_to_soil: Vec<MapD> = vec![];
    let mut soil_to_fertilizer: Vec<MapD> = vec![];
    let mut fertilizer_to_water: Vec<MapD> = vec![];
    let mut water_to_light: Vec<MapD> = vec![];
    let mut light_to_temperature: Vec<MapD> = vec![];
    let mut temperature_to_humidity: Vec<MapD> = vec![];
    let mut humidity_to_location: Vec<MapD> = vec![];

    //remove empty line
    lines.next();

    while let Some(line) = lines.next() {
        let map_name: &str = line.split_whitespace().next().unwrap();
        let corresponding_vec = match map_name {
            "seed-to-soil" => &mut seed_to_soil,
            "soil-to-fertilizer" => &mut soil_to_fertilizer,
            "fertilizer-to-water" => &mut fertilizer_to_water,
            "water-to-light" => &mut water_to_light,
            "light-to-temperature" => &mut light_to_temperature,
            "temperature-to-humidity" => &mut temperature_to_humidity,
            "humidity-to-location" => &mut humidity_to_location,
            _ => panic!(),
        };
        update_vec(&mut lines, corresponding_vec);
    }

    Input {
        seeds,
        seed_to_soil,
        soil_to_fertilizer,
        fertilizer_to_water,
        water_to_light,
        light_to_temperature,
        temperature_to_humidity,
        humidity_to_location,
    }
}

fn update_vec(lines: &mut Lines, corresponding_vec: &mut Vec<MapD>) {
    for line in lines.by_ref() {
        if line.trim().is_empty() {
            break;
        }
        let mut split_line = line.split_whitespace();
        let destination_range_start = split_line.next().unwrap().parse::<usize>().unwrap();
        let source_range_start = split_line.next().unwrap().parse::<usize>().unwrap();
        let range_length = split_line.next().unwrap().parse::<usize>().unwrap();
        corresponding_vec.push(MapD {
            destination_range_start,
            source_range_start,
            range_length,
        });
    }
}
