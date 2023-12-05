mod part1;
mod part2;

use std::error::Error;
use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() -> Result<(), Box<dyn Error>> {
    part1::part1()?;
    part2::part2()?;

    Ok(())
}
