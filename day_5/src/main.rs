use std::fmt::Display;
use std::fs::*;

use nom::{
    bytes::complete::tag, character::complete::space0, character::complete::space1,
    character::complete::u64, combinator::opt, multi::many0, sequence::preceded,
    sequence::terminated, sequence::tuple, IResult,
};

fn main() {
    let input = read_to_string("./input.txt").expect("File does not exist");

    let splits: Vec<&str> = input.split("\n\n").collect();
    let (_, seeds) = parse_seeds(splits[0]).unwrap();
    let maps: Data = load_maps(&splits);

    println!("part 1: {}", min_location(&seeds, &maps));
    println!("part 2: {}", min_location2(&seeds, &maps));
}

fn load_maps(splits: &Vec<&str>) -> Data {
    let (_, seed_to_soil) = parse_map(String::from("seed-to-soil map:\n"), splits[1]).unwrap();

    let (_, soil_to_fertilizer) =
        parse_map(String::from("soil-to-fertilizer map:\n"), splits[2]).unwrap();

    let (_, fertilizer_to_water) =
        parse_map(String::from("fertilizer-to-water map:\n"), splits[3]).unwrap();

    let (_, water_to_light) = parse_map(String::from("water-to-light map:\n"), splits[4]).unwrap();

    let (_, light_to_temperature) =
        parse_map(String::from("light-to-temperature map:\n"), splits[5]).unwrap();

    let (_, temperature_to_humidity) =
        parse_map(String::from("temperature-to-humidity map:\n"), splits[6]).unwrap();

    let (_, humidity_to_location) =
        parse_map(String::from("humidity-to-location map:\n"), splits[7]).unwrap();

    return Data {
        seed_to_soil,
        soil_to_fertilizer,
        fertilizer_to_water,
        water_to_light,
        light_to_temperature,
        temperature_to_humidity,
        humidity_to_location,
    };
}

fn min_location(seeds: &Vec<u64>, maps: &Data) -> u64 {
    let mut min_location: u64 = u64::MAX;

    for s in seeds {
        let soil = lookup(*s, &maps.seed_to_soil);
        let fertilizer = lookup(soil, &maps.soil_to_fertilizer);
        let water = lookup(fertilizer, &maps.fertilizer_to_water);
        let light = lookup(water, &maps.water_to_light);
        let temperature = lookup(light, &maps.light_to_temperature);
        let humidity = lookup(temperature, &maps.temperature_to_humidity);
        let location = lookup(humidity, &maps.humidity_to_location);

        if location < min_location {
            min_location = location;
        }
    }
    return min_location;
}

fn min_location2(seeds: &Vec<u64>, maps: &Data) -> u64 {
    let mut sranges: Vec<(u64, u64)> = seeds.chunks(2).map(|v| (v[0], v[0] + v[1])).collect();
    let mranges = vec![
        maps.seed_to_soil.clone(),
        maps.soil_to_fertilizer.clone(),
        maps.fertilizer_to_water.clone(),
        maps.water_to_light.clone(),
        maps.light_to_temperature.clone(),
        maps.temperature_to_humidity.clone(),
        maps.humidity_to_location.clone(),
    ];

    for m in mranges.iter() {
        let mut locs: Vec<(u64, u64)> = Vec::new();

        while let Some((s, e)) = sranges.pop() {
            let mut found: bool = false;

            for (dest, start, c) in m.into_iter() {
                let os = u64::max(s, *start);
                let oe = u64::min(e, *start + c);

                if os < oe {
                    locs.push((os - start + dest, oe - start + dest));
                    if os > s {
                        sranges.push((s, os));
                    }
                    if e < oe {
                        sranges.push((oe, e));
                    }
                    found = true;
                    break;
                }
            }
            if !found {
                locs.push((s, e));
            }
        }
        sranges = locs.clone();
    }

    sranges.sort();
    return sranges[0].0;
}

fn lookup(input: u64, lookup: &Vec<(u64, u64, u64)>) -> u64 {
    for (dest, source, n) in lookup {
        if input >= *source && input < *source + *n {
            return *dest + input - *source;
        }
    }
    return input;
}

fn parse_seeds(s: &str) -> IResult<&str, Vec<u64>> {
    return preceded(tag("seeds:"), many0(preceded(space0, u64)))(s);
}

fn parse_map(name: String, s: &str) -> IResult<&str, Vec<(u64, u64, u64)>> {
    return preceded(
        tag(name.as_str()),
        many0(tuple((
            terminated(u64, space1),
            terminated(u64, space1),
            terminated(u64, opt(tag("\n"))),
        ))),
    )(s);
}

#[derive(Clone, Debug, PartialEq)]
struct Data {
    seed_to_soil: Vec<(u64, u64, u64)>,
    soil_to_fertilizer: Vec<(u64, u64, u64)>,
    fertilizer_to_water: Vec<(u64, u64, u64)>,
    water_to_light: Vec<(u64, u64, u64)>,
    light_to_temperature: Vec<(u64, u64, u64)>,
    temperature_to_humidity: Vec<(u64, u64, u64)>,
    humidity_to_location: Vec<(u64, u64, u64)>,
}

impl Display for Data {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.seed_to_soil)
    }
}
