use std::fs;

use day4::camp_cleanup::*;

fn main() -> std::io::Result<()> {
    let input = fs::read_to_string("./data/day4.txt")?;
    let lines = input.lines();
    // println!("{lines:?}");

    let assignment_str_pairs: Vec<Vec<&str>> = lines
        .map(|line| line.split(",").collect::<Vec<&str>>())
        .collect();
    // println!("{assignment_str_pairs:?}");

    let assignment_pairs: Vec<Vec<Assignment>> = assignment_str_pairs
        .iter()
        .map(|pair| pair.iter().filter_map(|&s| s.parse().ok()).collect())
        .collect();
    // println!("{assignment_pairs:?}");

    let containment_assignments: Vec<&Vec<Assignment>> = assignment_pairs
        .iter()
        .filter(|pair| has_containment(pair.get(0).unwrap(), pair.get(1).unwrap()))
        .collect();
    // println!("{containment_assignments:?}");

    let containment_count = containment_assignments.len();
    println!("Part 1: {containment_count}");

    let overlapping_assignments: Vec<&Vec<Assignment>> = assignment_pairs
        .iter()
        .filter(|pair| has_overlap(pair.get(0).unwrap(), pair.get(1).unwrap()))
        .collect();
    // println!("{overlapping_assignments:?}");

    let overlapping_count = overlapping_assignments.len();
    println!("Part 2: {overlapping_count}");

    Ok(())
}
