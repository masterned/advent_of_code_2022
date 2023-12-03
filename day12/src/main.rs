use std::{error::Error, fs};

use day12::atlas::Atlas;

fn main() -> Result<(), Box<dyn Error>> {
    let atlas: Atlas = fs::read_to_string("./data/day12")?.parse()?;
    let fewest_steps = atlas.count_fewest_steps_from_start_to_end()?;

    println!("Part 1: {fewest_steps}");

    Ok(())
}
