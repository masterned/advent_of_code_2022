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

#[must_use]
pub fn get_hand_from_opponent_and_result(opponent: &Hand, result: &RoundResult) -> Hand {
    use Hand::{Paper, Rock, Scissors};

    match result {
        RoundResult::Draw => match opponent {
            Rock => Rock,
            Paper => Paper,
            Scissors => Scissors,
        },
        RoundResult::Lose => match opponent {
            Rock => Scissors,
            Paper => Rock,
            Scissors => Paper,
        },
        RoundResult::Win => match opponent {
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
    #[must_use]
    pub fn from_hand_hand((hand1, hand2): &(&str, &str)) -> Option<Self> {
        if let Ok(hand1) = hand1.parse::<Hand>() {
            if let Ok(hand2) = hand2.parse::<Hand>() {
                return Some(Round(hand1, hand2));
            }
        }
        None
    }

    #[must_use]
    pub fn from_hand_result((hand, result): &(&str, &str)) -> Option<Self> {
        if let Ok(opponent) = hand.parse() {
            if let Ok(result) = result.parse() {
                let player = get_hand_from_opponent_and_result(&opponent, &result);
                return Some(Round(opponent, player));
            }
        }
        None
    }

    #[must_use]
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
        let players = s.split(' ').collect::<Vec<&str>>();

        if players.len() != 2 {
            return Err(Self::Err {});
        }

        if let (Some(first_player), Some(second_player)) = (players.first(), players.get(1)) {
            let first_player = first_player.parse().unwrap();
            let second_player = second_player.parse().unwrap();
            Ok(Self(first_player, second_player))
        } else {
            Err(Self::Err {})
        }
    }
}
