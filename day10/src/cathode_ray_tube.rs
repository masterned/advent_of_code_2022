use std::{
    collections::{hash_map::Entry, HashMap},
    str::FromStr,
};

#[derive(Debug)]
pub struct CPU {
    x_reg: i32,
    cycle: i32,
    interesting_signal_strengths: HashMap<i32, i32>,
}

impl CPU {
    pub fn set_interesting_signal_strengths(&mut self, interesting_signal_strengths: Vec<i32>) {
        for interesting_signal_strength in interesting_signal_strengths.iter() {
            self.interesting_signal_strengths
                .insert(*interesting_signal_strength, -1);
        }
    }

    pub fn get_x_register_value(&self) -> i32 {
        self.x_reg
    }

    fn insert_if_interesting(&mut self) {
        if let Entry::Occupied(mut entry) = self.interesting_signal_strengths.entry(self.cycle) {
            let signal_strength = self.cycle * self.x_reg;
            entry.insert(signal_strength);
        }
    }

    pub fn execute(&mut self, instruction: &Instruction) {
        if let Instruction::Addx(amount) = instruction {
            self.cycle += 1;
            self.insert_if_interesting();
            self.x_reg += amount;
        }

        self.cycle += 1;
        self.insert_if_interesting();
    }

    pub fn sum_interesting_signal_strengths(&self) -> i32 {
        self.interesting_signal_strengths.values().sum()
    }
}

impl Default for CPU {
    fn default() -> Self {
        CPU {
            x_reg: 1,
            cycle: 1,
            interesting_signal_strengths: HashMap::new(),
        }
    }
}

#[derive(Debug)]
pub enum Instruction {
    Addx(i32),
    Noop,
}

pub struct ParseInstructionError;

impl FromStr for Instruction {
    type Err = ParseInstructionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tokens = s.split_whitespace();

        match tokens.next().ok_or(ParseInstructionError)? {
            "addx" => {
                let amount = tokens
                    .next()
                    .ok_or(ParseInstructionError)?
                    .parse()
                    .map_err(|_| ParseInstructionError)?;
                Ok(Self::Addx(amount))
            }
            "noop" => Ok(Self::Noop),
            _ => Err(ParseInstructionError),
        }
    }
}
