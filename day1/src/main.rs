use std::{error::Error, fs};

use day1::calorie_counting::{get_top_3_heaviest_elves, get_total_elves_weight, Troup};

fn main() -> Result<(), Box<dyn Error>> {
    let file = fs::read_to_string("./data/day1.txt")?;
    let lines: Vec<&str> = file.lines().collect();

    let mut troup = Troup::new(&lines);

    let heaviest_elf = troup
        .get_heaviest_elf()
        .ok_or("No heaviest elf if no elves.")?;
    let total_calories = heaviest_elf.total_calories();
    println!("Part 1: {total_calories}");

    troup.add_elf(heaviest_elf);
    let top_3 = get_top_3_heaviest_elves(&mut troup);
    let total_weight = get_total_elves_weight(&top_3);
    println!("Part 2: {total_weight}");

    Ok(())
}
