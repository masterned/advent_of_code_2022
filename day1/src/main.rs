use day1::calorie_counting::*;
use input_reader::read_file_as_lines;

fn main() {
    let lines = read_file_as_lines("./data/day1.txt");
    let mut elves = build_elf_vec(&lines.unwrap());

    let heaviest_elf = get_heaviest_elf(&elves);
    println!("Part 1: {}", heaviest_elf.unwrap().total_calories());

    let top_3 = get_top_3_heaviest_elves(&mut elves);
    let total_weight = get_total_elves_weight(top_3.unwrap().to_vec().as_ref());
    println!("Part 2: {total_weight}");
}
