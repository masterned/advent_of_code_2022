use day3::rucksack_reorganization::*;
use input_reader::read_file_as_lines;

fn main() {
    let lines = read_file_as_lines("./data/day3.txt").unwrap();

    let pairs: Vec<(&str, &str)> = lines.iter().map(|line| pair_line(line)).collect();
    // println!("{pairs:?}");

    let commonalities: Vec<String> = pairs
        .iter()
        .map(|(comp1, comp2)| find_commonality(comp1, comp2))
        .collect();
    // println!("{commonalities:?}");

    let priorities: Vec<i32> = commonalities
        .iter()
        .map(|items| get_priority(items))
        .collect();
    // println!("{priorities:?}");

    let total_priorities: i32 = priorities.iter().sum();
    println!("Part 1: {total_priorities}");
}
