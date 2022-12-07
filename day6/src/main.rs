use day6::tuning_trouble::*;
use std::{fs, io};

fn main() -> io::Result<()> {
    let input = fs::read_to_string("./data/day6.txt")?;

    let start_of_packet = find_start_of_packet(&input);
    println!("Part 1: {start_of_packet}");

    let start_of_message = find_start_of_message(&input);
    println!("Part 2: {start_of_message}");

    Ok(())
}
