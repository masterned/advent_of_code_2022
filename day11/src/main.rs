use day11::monkey_in_the_middle::*;
use std::{error::Error, fs};

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("./data/day11")?;

    let blocks = input.split("\n\n");

    let monkeys: Vec<Monkey> = blocks.filter_map(|block| block.parse().ok()).collect();
    // println!("{monkeys:?}");

    let mut troup = Troup::default();
    for monkey in monkeys {
        troup.add_monkey(monkey);
    }

    for _ in 0..20 {
        troup.play();
    }

    let monkey_business = troup.calculate_monkey_business();
    println!("Part 1: {monkey_business}");

    Ok(())
}
