use std::error::Error;

mod part2;
mod part1;



fn main() -> Result<(), Box<dyn Error>> {
    part1::part1()?;
    part2::part2()?;

    Ok(())
}