#[derive(Debug, Default, PartialEq)]
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
