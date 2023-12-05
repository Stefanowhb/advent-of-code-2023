use std::error::Error;
use std::fs::File;
use std::io::{prelude::*, BufReader};

#[derive(Debug)]
struct Map {
    ranges: Vec<MapRange>
}

impl Map {
    fn new() -> Self {
        Self {
            ranges: vec![],
        }
    }
}

#[derive(Debug)]
struct MapRange {
    destination_range_start: usize,
    source_range_start: usize,
    range_length: usize,
}

impl MapRange {
    fn new(destination_range_start: usize, source_range_start: usize, range_length: usize) -> Self {
        Self {
            destination_range_start,
            source_range_start,
            range_length,
        }
    }
}

fn parse_input(file: File) -> Result<(Vec<usize>, Map, Map, Map, Map, Map, Map, Map), Box<dyn Error>> {
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines()
        .map(|r| r.expect("Failed to read line"))
        .collect();
    let mut lines_iter = lines.iter();
    let seeds: Vec<usize> = lines_iter.next()
        .expect("Invalid file format")
        .split(": ")
        .last()
        .expect("Invalid file format")
        .split(" ")
        .map(|n| n.parse::<usize>().expect("Invalid seeds"))
        .collect();

    let mut maps: Vec<Map> = lines_iter.fold(Vec::new(), |mut acc, x| {
        if x.is_empty() {
            return acc;
        } else if x.ends_with("map:") {
            acc.push(Map::new());
            return acc;
        }
        let mut map = acc.last_mut().expect("Map should exist");

        let numbers: Vec<usize> = x.split(" ").map(|n| n.parse::<usize>().expect("Invalid number")).collect();
        assert_eq!(numbers.len(), 3); // Assert we have 3 numbers

        map.ranges.push(MapRange::new(numbers[0], numbers[1], numbers[2]));

        acc
    });
    assert_eq!(maps.len(), 7); // Assert that we got 7 maps

    Ok((seeds, maps.remove(0), maps.remove(0), maps.remove(0), maps.remove(0), maps.remove(0), maps.remove(0), maps.remove(0)))
}

fn get_destination(seed: usize, map: &Map) -> usize {
    let range = map.ranges
        .iter()
        .find(|range| {
            seed >= range.source_range_start && seed < range.source_range_start + range.range_length
        });

    if let Some(range) = range {
        range.destination_range_start + (seed - range.source_range_start)
    } else {
        seed
    }
}

pub fn part1() -> Result<(), Box<dyn Error>> {
    let file = File::open("day_5_input.txt")?;

    let (seeds, seed_to_soil, soil_to_fertilizer, fertilizer_to_water, water_to_light, light_to_temperature, temperature_to_humidity, humidity_to_location) = parse_input(file)?;
    let mut locations = vec![];


    for seed in seeds {
        let soil = get_destination(seed, &seed_to_soil);
        let fertilizer = get_destination(soil, &soil_to_fertilizer);
        let water = get_destination(fertilizer, &fertilizer_to_water);
        let light = get_destination(water, &water_to_light);
        let temperature = get_destination(light, &light_to_temperature);
        let humidity = get_destination(temperature, &temperature_to_humidity);
        let location = get_destination(humidity, &humidity_to_location);

        locations.push(location);

        // println!("Seed {seed}, soil {soil}, fertilizer {fertilizer}, water {water}, light {light}, temperature {temperature}, humidity {humidity}, location {location}");
    }

    println!("Part 1: The lowest location number is {}", locations.iter().min().expect("There should always be a result"));

    Ok(())
}