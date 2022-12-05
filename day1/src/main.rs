use day1::calorie_counting::*;
use input_reader::read_file_as_lines;

fn main() {
    let lines = read_file_as_lines("./data/day1.txt").unwrap();
    let mut troup = Troup::new(&lines);
    let heaviest_elf = troup.get_heaviest_elf().unwrap();
    println!("Part 1: {:?}", heaviest_elf.total_calories());

    troup.add_elf(heaviest_elf);
    let top_3 = get_top_3_heaviest_elves(&mut troup);
    let total_weight = get_total_elves_weight(&top_3);
    println!("Part 2: {total_weight}");
}
