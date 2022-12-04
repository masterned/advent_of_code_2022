use std::{cmp::Ordering, str::FromStr};

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Hand {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl Hand {}

impl Eq for Hand {}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        match self {
            Hand::Rock => match other {
                Hand::Rock => Ordering::Equal,
                Hand::Paper => Ordering::Less,
                Hand::Scissors => Ordering::Greater,
            },
            Hand::Paper => match other {
                Hand::Rock => Ordering::Greater,
                Hand::Paper => Ordering::Equal,
                Hand::Scissors => Ordering::Less,
            },
            Hand::Scissors => match other {
                Hand::Rock => Ordering::Less,
                Hand::Paper => Ordering::Greater,
                Hand::Scissors => Ordering::Equal,
            },
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct ParseHandError {}

impl FromStr for Hand {
    type Err = ParseHandError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "A" | "X" => Ok(Hand::Rock),
            "B" | "Y" => Ok(Hand::Paper),
            "C" | "Z" => Ok(Hand::Scissors),
            _ => Err(ParseHandError {}),
        }
    }
}

#[derive(Debug)]
pub struct Round(Hand, Hand);

impl Round {
    pub fn get_score(&self) -> i32 {
        self.1 as i32
            + (match self.1.cmp(&self.0) {
                Ordering::Less => 0,
                Ordering::Equal => 3,
                Ordering::Greater => 6,
            })
    }
}

pub struct ParseRoundError;

impl FromStr for Round {
    type Err = ParseRoundError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let players = s.split(" ").collect::<Vec<&str>>();

        if players.len() != 2 {
            return Err(ParseRoundError {});
        }

        if let (Some(player1), Some(player2)) = (players.get(0), players.get(1)) {
            let player1 = player1.parse().unwrap();
            let player2 = player2.parse().unwrap();
            Ok(Round(player1, player2))
        } else {
            Err(ParseRoundError {})
        }
    }
}
