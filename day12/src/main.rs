use std::{error::Error, fs};

use day12::atlas::Atlas;

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("./data/day12")?;
    let lines: Vec<&str> = input.lines().collect();
    let atlas = Atlas::from(lines);

    let start = atlas.start.ok_or_else(|| "Unable to find start")?;
    let end = atlas.end.ok_or_else(|| "Unable to find end")?;

    let fewest_steps = atlas
        .count_fewest_steps(start, end)
        .ok_or_else(|| "Unable to find route")?;

    println!("483 is too high");
    println!("Part 1: {fewest_steps}");

    Ok(())
}
