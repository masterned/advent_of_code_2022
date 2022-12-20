use std::str::FromStr;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Test {
    pub divisible_value: usize,
    pub true_monkey_number: usize,
    pub false_monkey_number: usize,
}

impl Test {
    pub fn compare_against(&self, value: usize) -> usize {
        if value % self.divisible_value == 0 {
            self.true_monkey_number
        } else {
            self.false_monkey_number
        }
    }
}

#[derive(Debug)]
pub struct ParseTestError;

impl FromStr for Test {
    type Err = ParseTestError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();

        let divisibility_line = lines.next().ok_or(ParseTestError)?.trim();
        let mut tokens = divisibility_line.split_whitespace().skip(3);
        let divisible_value = tokens
            .next()
            .ok_or(ParseTestError)?
            .parse()
            .map_err(|_| ParseTestError)?;

        let true_monkey_line = lines.next().ok_or(ParseTestError)?.trim();
        let mut tokens = true_monkey_line.split_whitespace().skip(5);
        let true_monkey_number = tokens
            .next()
            .ok_or(ParseTestError)?
            .parse()
            .map_err(|_| ParseTestError)?;

        let false_monkey_line = lines.next().ok_or(ParseTestError)?.trim();
        let mut tokens = false_monkey_line.split_whitespace().skip(5);
        let false_monkey_number = tokens
            .next()
            .ok_or(ParseTestError)?
            .parse()
            .map_err(|_| ParseTestError)?;

        Ok(Test {
            divisible_value,
            true_monkey_number,
            false_monkey_number,
        })
    }
}

impl Default for Test {
    fn default() -> Self {
        Self {
            divisible_value: 1,
            true_monkey_number: 0,
            false_monkey_number: 0,
        }
    }
}
