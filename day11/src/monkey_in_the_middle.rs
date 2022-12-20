use std::{collections::VecDeque, str::FromStr};

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
    sign: Sign,
    left: Operator,
    right: Operator,
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Test {
    divisible_value: usize,
    true_monkey_number: usize,
    false_monkey_number: usize,
}

impl Test {
    fn compare_against(&self, value: usize) -> usize {
        if value % self.divisible_value == 0 {
            self.true_monkey_number
        } else {
            self.false_monkey_number
        }
    }
}

#[derive(Debug)]
struct ParseTestError;

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

#[derive(Debug)]
pub struct Monkey {
    inspection_count: usize,
    items: VecDeque<usize>,
    operation: Operation,
    test: Test,
}

#[derive(Debug)]
pub enum ParseMonkeyError {
    MissingToken,
    ItemsError,
    OperationError,
    TestError,
}

impl Monkey {
    fn parse_items(items_line: &str) -> Result<VecDeque<usize>, ParseMonkeyError> {
        let items_line = items_line.trim().replace(',', "");
        let tokens = items_line.split_whitespace().skip(2);

        let mut items = VecDeque::new();
        for item in tokens {
            let item = item.parse().map_err(|_| ParseMonkeyError::ItemsError)?;
            items.push_back(item);
        }
        Ok(items)
    }

    fn parse_operation(operation_line: &str) -> Result<Operation, ParseMonkeyError> {
        let operation_line = operation_line.trim();
        let tokens: Vec<&str> = operation_line.split_whitespace().skip(3).collect();
        let operation_line = tokens.join(" ");

        operation_line
            .parse()
            .map_err(|_| ParseMonkeyError::OperationError)
    }

    fn parse_test(items_line: &str) -> Result<Test, ParseMonkeyError> {
        items_line.parse().map_err(|_| ParseMonkeyError::TestError)
    }

    pub fn get_inspection_count(&self) -> usize {
        self.inspection_count
    }

    pub fn receive(&mut self, item: usize) {
        self.items.push_back(item);
    }

    pub fn interact_with(&self, item: usize) -> (usize, usize) {
        let new_item = self.operation.perform_on(item) / 3;

        let next_monkey = self.test.compare_against(new_item);

        (new_item, next_monkey)
    }

    pub fn take_turn(&mut self) -> VecDeque<(usize, usize)> {
        let mut monkey_moves = VecDeque::new();
        for _ in 0..self.items.len() {
            self.inspection_count += 1;
            if let Some(item) = self.items.pop_front() {
                monkey_moves.push_back(self.interact_with(item));
            }
        }
        monkey_moves
    }
}

impl FromStr for Monkey {
    type Err = ParseMonkeyError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let _monkey_id_line = lines.next().ok_or(ParseMonkeyError::MissingToken)?;

        let items_line = lines.next().ok_or(ParseMonkeyError::MissingToken)?;
        let items = Monkey::parse_items(items_line)?;

        let operation_line = lines.next().ok_or(ParseMonkeyError::MissingToken)?;
        let operation = Monkey::parse_operation(operation_line)?;

        let test_line = lines.collect::<Vec<&str>>().join("\n");
        let test = Monkey::parse_test(&test_line)?;

        Ok(Self {
            inspection_count: 0,
            items,
            operation,
            test,
        })
    }
}

#[derive(Debug, Default)]
pub struct Troup {
    monkeys: VecDeque<Monkey>,
}

impl Troup {
    pub fn add_monkey(&mut self, monkey: Monkey) {
        self.monkeys.push_back(monkey);
    }

    pub fn play(&mut self) {
        for index in 0..self.monkeys.len() {
            if let Some(monkey) = self.monkeys.get_mut(index) {
                let monkey_moves = monkey.take_turn();
                monkey_moves.iter().for_each(|&(item, monkey_number)| {
                    self.monkeys[monkey_number].receive(item);
                });
            }
        }
    }

    pub fn calculate_monkey_business(&self) -> usize {
        let mut inspection_counts: Vec<usize> = self
            .monkeys
            .iter()
            .map(|monkey| monkey.get_inspection_count())
            .collect();
        inspection_counts.sort();
        inspection_counts.reverse();
        inspection_counts[0] * inspection_counts[1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn _should_parse_item_line() -> Result<(), ParseMonkeyError> {
        let items = Monkey::parse_items("  Starting items: 54, 65, 75, 74")?;
        assert_eq!(items, vec![54, 65, 75, 74]);

        Ok(())
    }

    #[test]
    fn _should_parse_operation_line() -> Result<(), ParseMonkeyError> {
        let operation = Monkey::parse_operation("  Operation: new = old * 19")?;
        assert_eq!(
            operation,
            Operation {
                left: Operator::Old,
                sign: Sign::Multiply,
                right: Operator::Num(19)
            }
        );

        Ok(())
    }

    #[test]
    fn _should_parse_test_block() -> Result<(), ParseMonkeyError> {
        let test = Monkey::parse_test("  Test: divisible by 23\n    If true: throw to monkey 2\n    If false: throw to monkey 3")?;
        assert_eq!(
            test,
            Test {
                divisible_value: 23,
                true_monkey_number: 2,
                false_monkey_number: 3
            }
        );

        Ok(())
    }

    #[test]
    fn _monkey_should_increase_inspection_count_per_item_on_its_turn(
    ) -> Result<(), ParseOperationError> {
        let mut monkey = Monkey {
            inspection_count: 0,
            items: VecDeque::from(vec![100, 220, 280]),
            operation: "old + 20".parse()?,
            test: Test::default(),
        };

        monkey.take_turn();

        assert_eq!(monkey.get_inspection_count(), 3);

        Ok(())
    }

    #[test]
    fn _monkey_should_modify_item_on_turn() -> Result<(), ParseOperationError> {
        let test = Test::default();
        let mut monkey = Monkey {
            inspection_count: 0,
            items: VecDeque::from(vec![100, 220, 280]),
            operation: "old + 20".parse()?,
            test,
        };

        let monkey_moves = monkey.take_turn();

        assert_eq!(monkey_moves, vec![(40, 0), (80, 0), (100, 0)]);

        Ok(())
    }

    #[test]
    fn _troup_should_play() -> Result<(), ParseOperationError> {
        let mut troup = Troup {
            monkeys: VecDeque::from(vec![
                Monkey {
                    inspection_count: 0,
                    items: VecDeque::from(vec![100, 220, 280]),
                    operation: "old + 20".parse()?,
                    test: Test {
                        true_monkey_number: 1,
                        ..Test::default()
                    },
                },
                Monkey {
                    inspection_count: 0,
                    items: VecDeque::from(vec![30]),
                    operation: "old * 2".parse()?,
                    test: Test::default(),
                },
            ]),
        };
        troup.play();

        println!("{troup:?}");

        assert_eq!(troup.monkeys.front().unwrap().get_inspection_count(), 3);
        assert_eq!(troup.monkeys.back().unwrap().get_inspection_count(), 4);

        Ok(())
    }

    #[test]
    fn _troup_should_calculate_monkey_business() -> Result<(), ParseOperationError> {
        let mut troup = Troup {
            monkeys: VecDeque::from(vec![
                Monkey {
                    inspection_count: 0,
                    items: VecDeque::from(vec![79, 98]),
                    operation: "old * 19".parse()?,
                    test: Test {
                        divisible_value: 23,
                        true_monkey_number: 2,
                        false_monkey_number: 3,
                    },
                },
                Monkey {
                    inspection_count: 0,
                    items: VecDeque::from(vec![54, 65, 75, 74]),
                    operation: "old + 6".parse()?,
                    test: Test {
                        divisible_value: 19,
                        true_monkey_number: 2,
                        false_monkey_number: 0,
                    },
                },
                Monkey {
                    inspection_count: 0,
                    items: VecDeque::from(vec![79, 60, 97]),
                    operation: "old * old".parse()?,
                    test: Test {
                        divisible_value: 13,
                        true_monkey_number: 1,
                        false_monkey_number: 3,
                    },
                },
                Monkey {
                    inspection_count: 0,
                    items: VecDeque::from(vec![74]),
                    operation: "old + 3".parse()?,
                    test: Test {
                        divisible_value: 17,
                        true_monkey_number: 0,
                        false_monkey_number: 1,
                    },
                },
            ]),
        };

        for _round in 0..20 {
            troup.play();
        }

        assert_eq!(troup.calculate_monkey_business(), 10605);

        Ok(())
    }
}
