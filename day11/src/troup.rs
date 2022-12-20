use std::collections::VecDeque;

use crate::monkey::Monkey;

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
    use crate::{
        operation::{Operation, Operator, Sign},
        test::Test,
    };

    use super::*;

    #[test]
    fn _should_play() {
        let mut troup = Troup {
            monkeys: VecDeque::from(vec![
                Monkey::new(
                    &[100, 220, 280],
                    Operation {
                        left: Operator::Old,
                        sign: Sign::Add,
                        right: Operator::Num(20),
                    },
                    Test {
                        true_monkey_number: 1,
                        ..Test::default()
                    },
                ),
                Monkey::new(
                    &[30],
                    Operation {
                        left: Operator::Old,
                        sign: Sign::Multiply,
                        right: Operator::Num(2),
                    },
                    Test::default(),
                ),
            ]),
        };
        troup.play();

        println!("{troup:?}");

        assert_eq!(troup.monkeys.front().unwrap().get_inspection_count(), 3);
        assert_eq!(troup.monkeys.back().unwrap().get_inspection_count(), 4);
    }

    #[test]
    fn _should_calculate_monkey_business() {
        let mut troup = Troup {
            monkeys: VecDeque::from(vec![
                Monkey::new(
                    &[79, 98],
                    Operation {
                        left: Operator::Old,
                        sign: Sign::Multiply,
                        right: Operator::Num(19),
                    },
                    Test {
                        divisible_value: 23,
                        true_monkey_number: 2,
                        false_monkey_number: 3,
                    },
                ),
                Monkey::new(
                    &[54, 65, 75, 74],
                    Operation {
                        left: Operator::Old,
                        sign: Sign::Add,
                        right: Operator::Num(6),
                    },
                    Test {
                        divisible_value: 19,
                        true_monkey_number: 2,
                        false_monkey_number: 0,
                    },
                ),
                Monkey::new(
                    &[79, 60, 97],
                    Operation {
                        left: Operator::Old,
                        sign: Sign::Multiply,
                        right: Operator::Old,
                    },
                    Test {
                        divisible_value: 13,
                        true_monkey_number: 1,
                        false_monkey_number: 3,
                    },
                ),
                Monkey::new(
                    &[74],
                    Operation {
                        left: Operator::Old,
                        sign: Sign::Add,
                        right: Operator::Num(3),
                    },
                    Test {
                        divisible_value: 17,
                        true_monkey_number: 0,
                        false_monkey_number: 1,
                    },
                ),
            ]),
        };

        for _round in 0..20 {
            troup.play();
        }

        assert_eq!(troup.calculate_monkey_business(), 10605);
    }
}
