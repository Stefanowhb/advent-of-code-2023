use std::error::Error;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::ops::Range;
use std::thread;
use std::time::Instant;

#[derive(Debug, Clone)]
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

#[derive(Debug, Copy, Clone)]
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

fn parse_input(file: File) -> Result<(Vec<Range<usize>>, Map, Map, Map, Map, Map, Map, Map), Box<dyn Error>> {
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

    let seeds: Vec<Range<usize>> = seeds.chunks_exact(2).map(|c| c[0]..(c[0] + c[1])).collect();

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

pub fn part2() -> Result<(), Box<dyn Error>> {
    let file = File::open("day_5_input.txt")?;

    let (seeds, seed_to_soil, soil_to_fertilizer, fertilizer_to_water, water_to_light, light_to_temperature, temperature_to_humidity, humidity_to_location) = parse_input(file)?;
    let mut locations = vec![];
    let mut thread_handles = vec![];


    let start = Instant::now();

    for seed_range in seeds {
        let seed_to_soil = seed_to_soil.clone();
        let soil_to_fertilizer = soil_to_fertilizer.clone();
        let fertilizer_to_water = fertilizer_to_water.clone();
        let water_to_light = water_to_light.clone();
        let light_to_temperature = light_to_temperature.clone();
        let temperature_to_humidity = temperature_to_humidity.clone();
        let humidity_to_location = humidity_to_location.clone();

        thread_handles.push(thread::spawn(move || {
            let mut locations = vec![];

            for seed in seed_range {
                let soil = get_destination(seed, &seed_to_soil);
                let fertilizer = get_destination(soil, &soil_to_fertilizer);
                let water = get_destination(fertilizer, &fertilizer_to_water);
                let light = get_destination(water, &water_to_light);
                let temperature = get_destination(light, &light_to_temperature);
                let humidity = get_destination(temperature, &temperature_to_humidity);
                let location = get_destination(humidity, &humidity_to_location);

                locations.push(location);
            }

            locations
        }));
    }

    for thread_handle in thread_handles {
        if let Ok(mut thread_locations) = thread_handle.join() {
            locations.append(&mut thread_locations);
        } else {
            println!("Error getting locations");
        }
    }

    let elapsed = start.elapsed().as_secs();

    println!("Part 2: The lowest location number is {}, took {elapsed} seconds", locations.iter().min().expect("There should always be a result"));

    Ok(())
}