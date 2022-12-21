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

    pub fn play_hard(&mut self, rounds: usize) {
        let troup_lcd = self.get_troup_lcd();
        for _ in 0..rounds {
            for index in 0..self.monkeys.len() {
                if let Some(monkey) = self.monkeys.get_mut(index) {
                    let monkey_moves = monkey.take_crazy_turn(troup_lcd);
                    monkey_moves.iter().for_each(|&(item, monkey_number)| {
                        self.monkeys[monkey_number].receive(item);
                    });
                }
            }
        }
    }

    pub fn get_inspection_counts(&self) -> Vec<usize> {
        self.monkeys
            .iter()
            .map(|monkey| monkey.get_inspection_count())
            .collect()
    }

    pub fn get_troup_lcd(&self) -> usize {
        self.monkeys
            .iter()
            .map(|monkey| monkey.get_test_divisible_value())
            .product()
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

    #[test]
    fn _should_play_hard() {
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

        troup.play_hard(1);
        assert_eq!(
            troup.get_inspection_counts(),
            vec![2, 4, 3, 6],
            "Round 1 is wrong"
        );

        troup.play_hard(19);
        assert_eq!(
            troup.get_inspection_counts(),
            vec![99, 97, 8, 103],
            "round 20 is wrong"
        );

        troup.play_hard(980);
        assert_eq!(
            troup.get_inspection_counts(),
            vec![5204, 4792, 199, 5192],
            "Round 1,000 is wrong"
        )
    }
}
