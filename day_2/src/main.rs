use std::error::Error;
use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() -> Result<(), Box<dyn Error>>  {
    let file = File::open("day_2_input.txt")?;
    let reader = BufReader::new(file);

    let lines: Vec<String> = reader.lines()
        .map(|r| r.expect("Failed to read line"))
        .collect();

    let mut result = 0usize;

    for line in &lines {
        let mut split = line.split(": ");
        let game_id: usize = split.next().unwrap().split_at(5).1.parse()?;
        let mut red = 0usize;
        let mut green = 0usize;
        let mut blue = 0usize;

        for set in split.next().unwrap().split("; ") {
            for roll in set.split(", ") {
                let [num, col]: [&str; 2] = roll.split(" ").collect::<Vec<&str>>().try_into().unwrap();
                let num: usize = num.parse().unwrap();

                match col {
                    "red" if num > red => red = num,
                    "green" if num > green => green = num,
                    "blue" if num > blue => blue = num,
                    _ => {}
                }
            }
        }

        result += red * green * blue;
    }

    println!("{}", result);

    Ok(())
}
