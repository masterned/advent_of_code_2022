use day2::rock_paper_scissors::*;
use input_reader::read_file_as_lines;

fn main() {
    let lines = read_file_as_lines("./data/day2.txt").unwrap();

    let pairs: Vec<(&str, &str)> = lines
        .iter()
        .filter_map(|line| {
            let pairs = line.split(" ").collect::<Vec<&str>>();

            if pairs.len() != 2 {
                return None;
            }

            if let (Some(item1), Some(item2)) = (pairs.get(0), pairs.get(1)) {
                Some((*item1, *item2))
            } else {
                None
            }
        })
        .collect();

    let rounds = pairs
        .iter()
        .map(|pair| Round::from_hand_hand(&pair))
        .collect::<Vec<Round>>();
    // println!("{rounds:?}");

    let scores = rounds
        .iter()
        .map(|round| round.get_score())
        .collect::<Vec<i32>>();
    // println!("{scores:?}");

    let total_score: i32 = scores.iter().sum();
    println!("Part 1: {total_score}");

    let rounds: Vec<Round> = pairs
        .iter()
        .map(|pair| Round::from_hand_result(&pair))
        .collect();

    let scores: Vec<i32> = rounds.iter().map(|round| round.get_score()).collect();

    let total_score: i32 = scores.iter().sum();
    println!("Part 2: {total_score}");
}
