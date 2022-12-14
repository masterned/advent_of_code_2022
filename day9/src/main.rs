use std::{error::Error, fs};

use day9::rope_bridge::*;

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("./data/day9.txt")?;

    let lines = input.lines();

    let movements: Vec<Movement> = lines.filter_map(|line| line.parse().ok()).collect();

    let mut rope = Rope::default();
    movements.iter().for_each(|m| rope.simulate(m));
    let tail_visited_locations_count = rope.count_total_tail_visited_locations();
    println!("Part 1: {tail_visited_locations_count}");

    let mut rope = Rope::new(10);
    movements.iter().for_each(|m| rope.simulate(m));
    let tail_visited_locations_count = rope.count_total_tail_visited_locations();
    println!("Part 2: {tail_visited_locations_count}");

    Ok(())
}
