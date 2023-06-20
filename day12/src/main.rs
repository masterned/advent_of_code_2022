use std::{error::Error, fs};

use day12::atlas::Atlas;

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("./data/day12")?;
    let lines: Vec<&str> = input.lines().collect();
    let atlas = Atlas::from(lines);

    let fewest_steps = atlas.count_fewest_steps_from_start_to_end()?;

    println!("483 is too high");
    println!("Part 1: {fewest_steps}");

    Ok(())
}
