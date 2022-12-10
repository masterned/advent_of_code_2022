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

    Ok(())
}
