use std::{error::Error, fs};

use day2::rock_paper_scissors::Round;

fn main() -> Result<(), Box<dyn Error>> {
    let file = fs::read_to_string("./data/day2.txt")?;
    let lines = file.lines();

    let pairs: Vec<(&str, &str)> = lines
        .filter_map(|line| {
            let pairs: Vec<&str> = line.split(' ').collect();

            if pairs.len() != 2 {
                return None;
            }

            if let (Some(item1), Some(item2)) = (pairs.first(), pairs.get(1)) {
                Some((*item1, *item2))
            } else {
                None
            }
        })
        .collect();

    let total_score: i32 = pairs
        .iter()
        .map(|pair| {
            if let Some(round) = Round::from_hand_hand(pair) {
                round
            } else {
                panic!("Unable to parse both hands")
            }
        })
        .map(|round| round.get_score())
        .sum();
    println!("Part 1: {total_score}");

    let total_score: i32 = pairs
        .iter()
        .map(|pair| {
            if let Some(round) = Round::from_hand_result(pair) {
                round
            } else {
                panic!("Unable to create round")
            }
        })
        .map(|round| round.get_score())
        .sum();
    println!("Part 2: {total_score}");

    Ok(())
}
