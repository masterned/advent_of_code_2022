use std::{collections::HashSet, str::FromStr};

#[derive(Debug)]
struct Knot {
    position: (i32, i32),
    visited_locations: HashSet<(i32, i32)>,
}

impl Knot {
    fn follow(&mut self, other: &Self) {
        if self.position == other.position {
            return;
        }

        let (x, y) = self.position;

        if other.position == (x, y - 2) {
            self.position = (x, y - 1);
        } else if other.position == (x + 1, y - 2) || other.position == (x + 2, y - 1) {
            self.position = (x + 1, y - 1);
        } else if other.position == (x + 2, y) {
            self.position = (x + 1, y)
        } else if other.position == (x + 2, y + 1) || other.position == (x + 1, y + 2) {
            self.position = (x + 1, y + 1);
        } else if other.position == (x, y + 2) {
            self.position = (x, y + 1);
        } else if other.position == (x - 1, y + 2) || other.position == (x - 2, y + 1) {
            self.position = (x - 1, y + 1);
        } else if other.position == (x - 2, y) {
            self.position = (x - 1, y);
        } else if other.position == (x - 2, y - 1) || other.position == (x - 1, y - 2) {
            self.position = (x - 1, y - 1);
        }

        self.visited_locations.insert(self.position);
    }

    fn change_horizontal(&mut self, amount: i32) {
        self.position.0 += amount;
        self.visited_locations.insert(self.position);
    }

    fn change_vertical(&mut self, amount: i32) {
        self.position.1 += amount;
        self.visited_locations.insert(self.position);
    }

    fn total_visited_locations(&self) -> i32 {
        self.visited_locations.len().try_into().unwrap()
    }
}

impl Default for Knot {
    fn default() -> Self {
        let position = (0, 0);
        Knot {
            position,
            visited_locations: HashSet::from([position]),
        }
    }
}

#[derive(Debug, Default)]
pub struct Rope {
    head: Knot,
    tail: Knot,
}

impl Rope {
    pub fn simulate(&mut self, Movement { direction, amount }: &Movement) {
        for _ in 0..*amount {
            match direction {
                Direction::Up => {
                    self.head.change_vertical(-1);
                    self.tail.follow(&self.head);
                }
                Direction::Right => {
                    self.head.change_horizontal(1);
                    self.tail.follow(&self.head);
                }
                Direction::Down => {
                    self.head.change_vertical(1);
                    self.tail.follow(&self.head);
                }
                Direction::Left => {
                    self.head.change_horizontal(-1);
                    self.tail.follow(&self.head);
                }
            }
        }
    }

    pub fn count_total_tail_visit_location(&self) -> i32 {
        self.tail.total_visited_locations()
    }
}

#[derive(Debug)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Debug)]
pub struct Movement {
    direction: Direction,
    amount: usize,
}

#[derive(Debug)]
pub struct MovementParseError {}

impl FromStr for Movement {
    type Err = MovementParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tokens = s.split_whitespace();

        let direction = tokens.next().ok_or(Self::Err {})?.to_uppercase();

        let amount = tokens
            .next()
            .ok_or(Self::Err {})?
            .parse()
            .map_err(|_| Self::Err {})?;

        match direction.as_ref() {
            "U" => Ok(Self {
                direction: Direction::Up,
                amount,
            }),
            "R" => Ok(Self {
                direction: Direction::Right,
                amount,
            }),
            "D" => Ok(Self {
                direction: Direction::Down,
                amount,
            }),
            "L" => Ok(Self {
                direction: Direction::Left,
                amount,
            }),
            _ => Err(Self::Err {}),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn _default_knot_should_have_position_of_0_0() {
        let knot = Knot::default();
        assert_eq!(knot.position, (0, 0));
    }

    #[test]
    fn _default_knot_should_only_contain_starting_position() {
        let knot = Knot::default();
        assert_eq!(knot.visited_locations, HashSet::from([(0, 0)]));
        assert_eq!(knot.total_visited_locations(), 1);
    }

    #[test]
    fn _simulation_should_move_head_and_tail() {
        let mut rope = Rope::default();
        rope.simulate(&Movement {
            direction: Direction::Right,
            amount: 3,
        });
        assert_eq!(rope.head.position, (3, 0));
        assert_eq!(rope.tail.position, (2, 0));
    }

    #[test]
    fn _tail_should_not_move_if_it_does_not_need_to() {
        let mut rope = Rope::default();

        rope.simulate(&Movement {
            direction: Direction::Right,
            amount: 1,
        });
        rope.simulate(&Movement {
            direction: Direction::Up,
            amount: 1,
        });
        rope.simulate(&Movement {
            direction: Direction::Left,
            amount: 2,
        });
        rope.simulate(&Movement {
            direction: Direction::Down,
            amount: 2,
        });

        assert_eq!(rope.count_total_tail_visit_location(), 1);
    }

    #[test]
    fn _tail_should_move_diagonally() {
        let mut rope = Rope::default();
        rope.simulate(&Movement {
            direction: Direction::Right,
            amount: 1,
        });
        rope.simulate(&Movement {
            direction: Direction::Up,
            amount: 2,
        });
        assert_eq!(rope.tail.position, (1, -1));

        let mut rope = Rope::default();
        rope.simulate(&Movement {
            direction: Direction::Up,
            amount: 1,
        });
        rope.simulate(&Movement {
            direction: Direction::Right,
            amount: 2,
        });
        assert_eq!(rope.tail.position, (1, -1));
    }
}
