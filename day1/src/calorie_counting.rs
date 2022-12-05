use std::collections::BinaryHeap;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
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

impl PartialOrd for Elf {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Elf {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.total_calories().cmp(&other.total_calories())
    }
}

#[derive(Debug, Default)]
pub struct Troup {
    elves: BinaryHeap<Elf>,
}

impl Troup {
    pub fn new(lines: &Vec<String>) -> Self {
        let mut elves = BinaryHeap::new();

        let mut next_elf = Elf::default();
        for line in lines {
            if line.is_empty() {
                elves.push(next_elf);
                next_elf = Elf::default();
            } else {
                let calories_value = line.parse::<i32>().unwrap();
                next_elf.add_calories(calories_value);
            }
        }

        Troup { elves }
    }

    pub fn get_heaviest_elf(&mut self) -> Option<Elf> {
        self.elves.pop()
    }

    pub fn add_elf(&mut self, elf: Elf) {
        self.elves.push(elf);
    }
}

pub fn get_top_3_heaviest_elves(troup: &mut Troup) -> Vec<Elf> {
    let mut elves = Vec::new();

    for _ in 0..3 {
        elves.push(troup.get_heaviest_elf().unwrap());
    }

    elves
}

pub fn get_total_elves_weight(elves: &[Elf]) -> i32 {
    elves.iter().fold(0, |acc, elf| acc + elf.total_calories())
}
