#[derive(Clone, Debug, Default, PartialEq)]
pub struct Elf {
    calories: Vec<i32>,
}

impl Elf {
    pub fn add_calories(&mut self, calories: i32) {
        self.calories.push(calories);
    }

    pub fn total_calories(&self) -> i32 {
        self.calories.iter().sum()
    }
}

pub fn get_heaviest_elf(elves: &Vec<Elf>) -> Option<&Elf> {
    elves
        .iter()
        .max_by(|elf1, elf2| elf1.total_calories().cmp(&elf2.total_calories()))
}

pub fn get_top_3_heaviest_elves(elves: &mut Vec<Elf>) -> Option<&[Elf]> {
    elves.sort_by(|a, b| b.total_calories().cmp(&a.total_calories()));
    elves.get(0..3)
}

pub fn get_total_elves_weight(elves: &Vec<Elf>) -> i32 {
    elves.iter().fold(0, |acc, elf| acc + elf.total_calories())
}

pub fn build_elf_vec(lines: &Vec<String>) -> Vec<Elf> {
    let mut elves = Vec::new();

    let mut next_elf = Elf::default();
    for line in lines {
        if line == "" {
            elves.push(next_elf);
            next_elf = Elf::default();
        } else {
            let calories_value = line.parse::<i32>().unwrap();
            next_elf.add_calories(calories_value);
        }
    }

    elves
}
