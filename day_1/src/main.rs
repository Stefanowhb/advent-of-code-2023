use std::error::Error;
use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("day_1_input.txt")?;
    let reader = BufReader::new(file);

    let a: usize = reader.lines()
        .map(|r| r.expect("Failed to read line"))
        .map(|l| {
            l.replace("one", "o1ne")
                .replace("two", "t2wo")
                .replace("three", "t3hree")
                .replace("four", "f4our")
                .replace("five", "f5ive")
                .replace("six", "s6ix")
                .replace("seven", "s7even")
                .replace("eight", "e8ight")
                .replace("nine", "n9ine")
        })
        .map(|l| l.chars().filter(|c| c.is_numeric()).collect::<Vec<char>>())
        .map(|v| format!("{}{}", v.first().unwrap(), v.last().unwrap()).parse::<usize>().unwrap())
        .sum();

    println!("{}", a);

    Ok(())
}
