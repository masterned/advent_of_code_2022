use std::{error::Error, fs};

use day10::cathode_ray_tube::*;

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("./data/day10")?;

    let lines = input.lines();

    let mut cpu = CPU::default();
    cpu.set_interesting_signal_strengths(vec![20, 60, 100, 140, 180, 220]);

    let instructions: Vec<Instruction> = lines.filter_map(|line| line.parse().ok()).collect();

    instructions
        .iter()
        .for_each(|instruction| cpu.execute(instruction));

    println!("Part 1: {}", cpu.sum_interesting_signal_strengths());
    Ok(())
}
