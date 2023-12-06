use std::cmp::min;
use std::ops::Range;
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

pub fn get_min_location_p_1(input: Input) -> usize {
    let mut res = usize::MAX;

    for mut seed in input.seeds {
        //dbg!(&seed);
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
        //dbg!(seed);
        res = min(res, seed);
    }
    res
}

pub fn get_min_location_p_2(input: Input) -> usize {
    let mut res = usize::MAX;

    let mut vec_number = vec![];
    for chunk in input.seeds.chunks(2) {
        match chunk {
            &[start, range] => {
                vec_number.push(start..start + range);
            }
            _ => panic!(),
        }
    }

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
        let mut new_vec_number = vec![];
        while !vec_number.is_empty() {
            // filter with get_range_intersection
            // option is can use m.destination_range_start - m.source_range_start
            // other have to be put back in vec_number to be processed again
            // when empty we go for next map

            let seed = vec_number.pop().unwrap();

            let count = map
                .iter()
                .find(|m| {
                    // map is all seed_to_soil then all soil_to_fertilizer then ...
                    let (inter, mut outside) = get_range_intersection(
                        &seed,
                        &(m.source_range_start..m.source_range_start + m.range_length),
                    );
                    if inter.is_some() {
                        let start =
                            inter.unwrap().start + m.destination_range_start - m.source_range_start;
                        let end =
                            inter.unwrap().end + m.destination_range_start - m.source_range_start;
                        new_vec_number.push(start..end);
                        vec_number.append(&mut outside);
                    };
                    !inter.is_some()
                })
                .count();
            // if no inter, we keep the same range
            if count == 0 {
                new_vec_number.push(seed);
            }
        }
        vec_number = new_vec_number;
    }
    res
}

fn get_range_intersection(
    range1: &Range<usize>,
    range2: &Range<usize>,
) -> (Option<Range<usize>>, Vec<Range<usize>>) {
    let intersection_start = range1.start.max(range2.start);
    let intersection_end = range1.end.min(range2.end);

    let mut outside: Vec<Range<usize>> = vec![];

    if intersection_start <= intersection_end {
        return (Some(intersection_start..intersection_end), outside);
    } else {
        outside.push(range1);
        return (None, outside);
    }
}

pub fn get_maps(input: &str) -> Input {
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
