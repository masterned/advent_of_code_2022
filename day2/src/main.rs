use day2::rock_paper_scissors::*;
use input_reader::read_file_as_lines;

fn main() {
    let lines = read_file_as_lines("./data/day2.txt").unwrap();

    let rounds = lines
        .iter()
        .filter_map(|line| line.parse::<Round>().ok())
        .collect::<Vec<Round>>();
    // println!("{rounds:?}");

    let scores = rounds
        .iter()
        .map(|round| round.get_score())
        .collect::<Vec<i32>>();
    // println!("{scores:?}");

    let total_score: i32 = scores.iter().sum();
    println!("Part 1: {total_score}");
}
