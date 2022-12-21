use std::{collections::VecDeque, str::FromStr};

use crate::{operation::Operation, test::Test};

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
    pub fn new(starting_items: &[usize], operation: Operation, test: Test) -> Self {
        Monkey {
            inspection_count: 0,
            items: VecDeque::from(starting_items.to_vec()),
            operation,
            test,
        }
    }

    pub fn parse_items(items_line: &str) -> Result<VecDeque<usize>, ParseMonkeyError> {
        let items_line = items_line.trim().replace(',', "");
        let tokens = items_line.split_whitespace().skip(2);

        let mut items = VecDeque::new();
        for item in tokens {
            let item = item.parse().map_err(|_| ParseMonkeyError::ItemsError)?;
            items.push_back(item);
        }
        Ok(items)
    }

    pub fn parse_operation(operation_line: &str) -> Result<Operation, ParseMonkeyError> {
        let operation_line = operation_line.trim();
        let tokens: Vec<&str> = operation_line.split_whitespace().skip(3).collect();
        let operation_line = tokens.join(" ");

        operation_line
            .parse()
            .map_err(|_| ParseMonkeyError::OperationError)
    }

    pub fn parse_test(items_line: &str) -> Result<Test, ParseMonkeyError> {
        items_line.parse().map_err(|_| ParseMonkeyError::TestError)
    }

    pub fn get_inspection_count(&self) -> usize {
        self.inspection_count
    }

    pub fn get_test_divisible_value(&self) -> usize {
        self.test.divisible_value
    }

    pub fn receive(&mut self, item: usize) {
        self.items.push_back(item);
    }

    pub fn take_turn(&mut self) -> VecDeque<(usize, usize)> {
        let reducer = 3;
        let mut monkey_moves = VecDeque::new();
        for _ in 0..self.items.len() {
            self.inspection_count += 1;
            if let Some(item) = self.items.pop_front() {
                let mut new_item = self.operation.perform_on(item);
                if new_item > reducer {
                    new_item /= reducer;
                }
                let next_monkey = self.test.compare_against(new_item);
                monkey_moves.push_back((new_item, next_monkey));
            }
        }
        monkey_moves
    }

    pub fn take_crazy_turn(&mut self, reducer: usize) -> VecDeque<(usize, usize)> {
        let mut monkey_moves = VecDeque::new();
        for _ in 0..self.items.len() {
            self.inspection_count += 1;
            if let Some(item) = self.items.pop_front() {
                let mut new_item = self.operation.perform_on(item);
                if new_item > reducer {
                    new_item %= reducer;
                }
                let next_monkey = self.test.compare_against(new_item);
                monkey_moves.push_back((new_item, next_monkey));
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

#[cfg(test)]
mod tests {
    use crate::operation::{Operator, Sign};

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
    fn _should_increase_inspection_count_per_item_on_its_turn() {
        let mut monkey = Monkey {
            inspection_count: 0,
            items: VecDeque::from(vec![100, 220, 280]),
            operation: Operation {
                left: Operator::Old,
                sign: Sign::Add,
                right: Operator::Num(20),
            },
            test: Test::default(),
        };

        monkey.take_turn();

        assert_eq!(monkey.get_inspection_count(), 3);
    }

    #[test]
    fn _should_modify_item_on_turn() {
        let mut monkey = Monkey {
            inspection_count: 0,
            items: VecDeque::from(vec![100, 220, 280]),
            operation: Operation {
                left: Operator::Old,
                sign: Sign::Add,
                right: Operator::Num(20),
            },
            test: Test::default(),
        };

        let monkey_moves = monkey.take_turn();

        assert_eq!(monkey_moves, vec![(40, 0), (80, 0), (100, 0)]);
    }
}
