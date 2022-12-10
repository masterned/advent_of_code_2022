use std::error::Error;

use day7::no_space_left_on_device::*;

fn main() -> Result<(), Box<dyn Error>> {
    let input = std::fs::read_to_string("./data/day7.txt")?;
    let input_lines = input.lines();

    let mut file_system = FileSystem::default();

    for line in input_lines {
        if let Ok(msg) = line.parse::<FileSystemMessage>() {
            file_system.exec(msg).unwrap();
        }
    }

    let count = file_system.total_dir_size_less_than_or_equal_to_100_000();
    println!("Part 1: {count}");

    let free_space = 70_000_000 - file_system.get_total_size();
    // println!("free space: {free_space}");

    let needed_space = 30_000_000 - free_space;
    // println!("needed_space: {needed_space}");

    let min_size = file_system.get_minimum_size_with_at_least(needed_space);
    println!("Part 2: {min_size}");

    Ok(())
}
