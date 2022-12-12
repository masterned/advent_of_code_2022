use std::{error::Error, fs};

use day9::rope_bridge::*;

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("./data/day9.txt")?;
    // println!("{input}");

    let lines = input.lines();

    let movements: Vec<Movement> = lines.filter_map(|line| line.parse().ok()).collect();
    // println!("{movements:?}");

    let mut rope = Rope::default();
    movements.iter().for_each(|m| rope.simulate(m));
    let tail_count = rope.count_total_tail_visit_location();
    println!("Part 1: {tail_count}");

    Ok(())
}
