use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
pub enum Sign {
    Multiply,
    Add,
}

impl FromStr for Sign {
    type Err = ParseOperationError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(Self::Add),
            "*" => Ok(Self::Multiply),
            _ => Err(ParseOperationError::SignError),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Operator {
    Old,
    Num(usize),
}

impl FromStr for Operator {
    type Err = ParseOperationError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "old" => Ok(Self::Old),
            num => {
                let num = num
                    .parse()
                    .map_err(|_| ParseOperationError::OperatorError)?;
                Ok(Self::Num(num))
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Operation {
    pub sign: Sign,
    pub left: Operator,
    pub right: Operator,
}

#[derive(Debug)]
pub enum ParseOperationError {
    SignError,
    OperatorError,
    MissingToken,
}

impl Operation {
    pub fn perform_on(&self, old: usize) -> usize {
        match self.sign {
            Sign::Add => match self.left {
                Operator::Num(left) => match self.right {
                    Operator::Num(right) => left + right,
                    Operator::Old => left + old,
                },
                Operator::Old => match self.right {
                    Operator::Num(right) => old + right,
                    Operator::Old => old + old,
                },
            },
            Sign::Multiply => match self.left {
                Operator::Num(left) => match self.right {
                    Operator::Num(right) => left * right,
                    Operator::Old => left * old,
                },
                Operator::Old => match self.right {
                    Operator::Num(right) => old * right,
                    Operator::Old => old * old,
                },
            },
        }
    }
}

impl FromStr for Operation {
    type Err = ParseOperationError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tokens = s.split_whitespace();

        let left = tokens
            .next()
            .ok_or(ParseOperationError::MissingToken)?
            .parse()?;

        let sign = tokens
            .next()
            .ok_or(ParseOperationError::MissingToken)?
            .parse()?;

        let right = tokens
            .next()
            .ok_or(ParseOperationError::MissingToken)?
            .parse()?;

        Ok(Operation { sign, left, right })
    }
}
