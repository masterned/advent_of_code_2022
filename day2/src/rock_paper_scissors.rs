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

pub fn get_hand_from_opponent_and_result(opponent: &Hand, result: &RoundResult) -> Hand {
    use Hand::*;
    use RoundResult::*;

    match result {
        Draw => match opponent {
            Rock => Rock,
            Paper => Paper,
            Scissors => Scissors,
        },
        Lose => match opponent {
            Rock => Scissors,
            Paper => Rock,
            Scissors => Paper,
        },
        Win => match opponent {
            Rock => Paper,
            Paper => Scissors,
            Scissors => Rock,
        },
    }
}

#[derive(Debug, PartialEq)]
pub struct ParseHandError {}

impl FromStr for Hand {
    type Err = ParseHandError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "A" | "X" => Ok(Self::Rock),
            "B" | "Y" => Ok(Self::Paper),
            "C" | "Z" => Ok(Self::Scissors),
            _ => Err(Self::Err {}),
        }
    }
}

pub enum RoundResult {
    Lose = 0,
    Draw = 3,
    Win = 6,
}

#[derive(Debug)]
pub struct ParseRoundResultError {}

impl FromStr for RoundResult {
    type Err = ParseRoundResultError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "X" => Ok(Self::Lose),
            "Y" => Ok(Self::Draw),
            "Z" => Ok(Self::Win),
            _ => Err(Self::Err {}),
        }
    }
}

#[derive(Debug)]
pub struct Round(Hand, Hand);

impl Round {
    pub fn from_hand_hand((hand1, hand2): &(&str, &str)) -> Self {
        Round(hand1.parse().unwrap(), hand2.parse().unwrap())
    }

    pub fn from_hand_result((hand, result): &(&str, &str)) -> Self {
        let opponent: Hand = hand.parse().unwrap();
        let player = get_hand_from_opponent_and_result(&opponent, &result.parse().unwrap());
        Round(opponent, player)
    }

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
            return Err(Self::Err {});
        }

        if let (Some(player1), Some(player2)) = (players.get(0), players.get(1)) {
            let player1 = player1.parse().unwrap();
            let player2 = player2.parse().unwrap();
            Ok(Self(player1, player2))
        } else {
            Err(Self::Err {})
        }
    }
}
