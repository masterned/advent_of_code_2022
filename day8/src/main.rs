use std::{error::Error, fs};

use day8::treetop_tree_house::*;

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("./data/day8.txt")?;

    let digit_matrix: Vec<Vec<i32>> = input
        .lines()
        .map(|line| {
            line.chars()
                .filter_map(|c| c.to_string().parse::<i32>().ok())
                .collect()
        })
        .collect();
    // println!("digit matrix: {digit_matrix:?}");

    let neighbourhood = create_neighbourhood(&digit_matrix);
    // println!("{neighbourhood:?}");

    let visible_tree_count = neighbourhood
        .iter()
        .filter(|(tree, neighbours)| is_visible(*tree, neighbours))
        .count();
    println!("Part 1: {visible_tree_count}");

    Ok(())
}
