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

fn parse_input(file: File) -> Result<Vec<Race> , Box<dyn Error>>{
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines()
        .map(|r| r.expect("Failed to read line"))
        .collect();
    assert_eq!(lines.len(), 2);
    let times: Vec<usize> = lines.first()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|n| n.parse::<usize>().expect("Invalid number"))
        .collect();
    let distances: Vec<usize> = lines.last()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|n| n.parse::<usize>().expect("Invalid number"))
        .collect();
    assert_eq!(times.len(), distances.len());
    let zipped = times.iter().zip(distances);
    let mut ret = vec![];

    for (time, distance) in zipped {
        ret.push(Race::new(*time, distance));
    }

    Ok(ret)
}

pub fn part1() -> Result<(), Box<dyn Error>> {
    let file = File::open("day_6_input.txt")?;

    let races = parse_input(file)?;
    let mut result = 0;

    for race in &races {
        let win_count = race.get_winning_times().len();

        if result == 0 {
            result = win_count;
        } else {
            result *= win_count;
        }
    }

    println!("Part 1 result: {}", result);

    Ok(())
}
