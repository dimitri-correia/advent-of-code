use std::cmp::min;
use std::ops::Range;
use std::str::Lines;

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
        res = min(res, seed);
    }
    res
}

pub fn get_min_location_p_2(input: Input) -> usize {
    let mut vec_number = init_vec(&input);

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
        let mut new_vec_number = Vec::new();
        'rng: while let Some(range) = vec_number.pop() {
            for m in map.iter() {
                let (inter, mut outside) = get_range_intersection_left_all(
                    &range,
                    &(m.source_range_start..m.source_range_start + m.range_length),
                );

                if let Some(inter) = inter {
                    let start = inter.start + m.destination_range_start - m.source_range_start;
                    let end = inter.end + m.destination_range_start - m.source_range_start;
                    new_vec_number.push(start..end);
                    vec_number.append(&mut outside);
                    continue 'rng;
                }
            }
            new_vec_number.push(range);
        }
        vec_number = new_vec_number;
    }

    vec_number
        .iter()
        .filter(|rng| rng != &&Range { start: 0, end: 0 })
        .map(|range| range.start)
        .min()
        .unwrap()
}

fn init_vec(input: &Input) -> Vec<Range<usize>> {
    input
        .seeds
        .chunks(2)
        .map(|chunk| match chunk {
            [start, range] => *start..(*start + *range),
            _ => unreachable!(),
        })
        .collect()
}

fn get_range_intersection_left_all(
    range1: &Range<usize>,
    range2: &Range<usize>,
) -> (Option<Range<usize>>, Vec<Range<usize>>) {
    let intersection_start = range1.start.max(range2.start);
    let intersection_end = range1.end.min(range2.end);

    let intersection = if intersection_start < intersection_end {
        Some(intersection_start..intersection_end)
    } else {
        None
    };

    let mut ranges_left_of_intersection = vec![];
    if intersection_start == range2.start {
        ranges_left_of_intersection.push(range1.start..range2.start.min(range1.end));
    }
    if intersection_end == range2.end {
        ranges_left_of_intersection.push(range2.end.max(range1.start)..range1.end);
    };

    (intersection, ranges_left_of_intersection)
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
            _ => unreachable!(),
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

#[cfg(test)]
mod tests {
    use super::*;

    /// Test the scenario where there is no intersection between two ranges.
    #[test]
    fn test_no_intersection() {
        let r1 = 0..2;
        let r2 = 3..5;
        let (a, b) = get_range_intersection_left_all(&r1, &r2);

        assert_eq!(a, None, "Expected no intersection, but found one");
        assert_eq!(b, vec![0..2], "Unexpected range in result");
    }

    /// Test the scenario where there is an intersection between two ranges.
    #[test]
    fn test_intersection() {
        let r1 = 0..8;
        let r2 = 3..5;
        let (a, b) = get_range_intersection_left_all(&r1, &r2);

        assert_eq!(a, Some(3..5), "Unexpected intersection range");
        assert_eq!(b, vec![0..3, 5..8], "Unexpected ranges in result");
    }

    /// Test the scenario where one range is entirely within the other.
    #[test]
    fn test_one_range_within_another() {
        let r1 = 3..5;
        let r2 = 0..8;
        let (a, b) = get_range_intersection_left_all(&r1, &r2);

        assert_eq!(a, Some(3..5), "Unexpected intersection range");
        assert_eq!(b, vec![], "Unexpected ranges in result");
    }
}
