use std::error::Error;
use std::fs::File;
use std::io::{prelude::*, BufReader};

struct Race {
    time: usize,
    record_distance: usize,
}

impl Race {
    fn new(time: usize, record_distance: usize) -> Self {
        Self {
            time,
            record_distance,
        }
    }

    fn get_winning_times(&self) -> Vec<usize> {
        let mut ret = vec![];

        for time_pressed in 0..=self.time {
            let speed = 1 * time_pressed;
            let time_left = self.time - time_pressed;
            let distance = speed * time_left;

            if distance > self.record_distance {
                ret.push(distance);
            }
        }

        ret
    }
}

fn parse_input(file: File) -> Result<Race , Box<dyn Error>>{
    let reader = BufReader::new(file);
    let mut lines: Vec<String> = reader.lines()
        .map(|r| r.expect("Failed to read line"))
        .collect();
    assert_eq!(lines.len(), 2);
    let mut time_line = lines.first()
        .unwrap()
        .clone();
    time_line.retain(|c| !c.is_whitespace());

    let mut distance_line = lines.last_mut()
        .unwrap()
        .clone();
    distance_line.retain(|c| !c.is_whitespace());

    let time: usize = time_line.split_once(":").expect("Invalid file format").1.parse().expect("Number");
    let record_distance: usize = distance_line.split_once(":").expect("Invalid file format").1.parse().expect("Number");

    Ok(Race {
        time,
        record_distance,
    })
}

pub fn part2() -> Result<(), Box<dyn Error>> {
    let file = File::open("day_6_input.txt")?;

    let race = parse_input(file)?;
    let mut result = race.get_winning_times().len();

    println!("Part 1 result: {}", result);

    Ok(())
}
