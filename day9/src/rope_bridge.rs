use std::{collections::HashSet, str::FromStr};

#[derive(Clone, Copy, Debug, Default)]
struct Knot {
    position: (i32, i32),
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
    }

    fn travel(&mut self, (delta_x, delta_y): (i32, i32)) {
        self.position.0 += delta_x;
        self.position.1 += delta_y;
    }
}

#[derive(Debug)]
pub struct Rope {
    knots: Vec<Knot>,
    visited_tail_locations: HashSet<(i32, i32)>,
}

impl Rope {
    pub fn new(num_knots: usize) -> Self {
        let mut knots = Vec::with_capacity(num_knots);

        for _ in 0..num_knots {
            knots.push(Knot::default());
        }

        Rope {
            knots,
            visited_tail_locations: HashSet::new(),
        }
    }

    fn move_head(&mut self, direction: &Direction) {
        let head = self.knots.first_mut().unwrap();
        match direction {
            Direction::Up => head.travel((0, -1)),
            Direction::Right => head.travel((1, 0)),
            Direction::Down => head.travel((0, 1)),
            Direction::Left => head.travel((-1, 0)),
        };
    }

    pub fn simulate(&mut self, Movement { direction, amount }: &Movement) {
        for _ in 0..*amount {
            self.move_head(direction);

            for i in 1..self.knots.len() {
                let leader = self.knots[i - 1];
                self.knots[i].follow(&leader);
            }

            self.visited_tail_locations.insert(self.get_tail_position());
        }
    }

    pub fn count_total_tail_visited_locations(&self) -> usize {
        self.visited_tail_locations.len()
    }

    pub fn get_head_position(&self) -> (i32, i32) {
        self.knots.first().unwrap().position
    }

    pub fn get_tail_position(&self) -> (i32, i32) {
        self.knots.last().unwrap().position
    }
}

impl Default for Rope {
    fn default() -> Self {
        let knots = vec![Knot::default(), Knot::default()];
        Rope {
            knots,
            visited_tail_locations: HashSet::new(),
        }
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
    fn _simulation_should_move_head_and_tail() {
        let mut rope = Rope::default();
        rope.simulate(&Movement {
            direction: Direction::Right,
            amount: 3,
        });
        assert_eq!(rope.get_head_position(), (3, 0));
        assert_eq!(rope.get_tail_position(), (2, 0));
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

        assert_eq!(rope.count_total_tail_visited_locations(), 1);
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
        assert_eq!(rope.get_tail_position(), (1, -1));

        let mut rope = Rope::default();
        rope.simulate(&Movement {
            direction: Direction::Up,
            amount: 1,
        });
        rope.simulate(&Movement {
            direction: Direction::Right,
            amount: 2,
        });
        assert_eq!(rope.get_tail_position(), (1, -1));
    }

    #[test]
    fn _tail_should_whip_on_diagonal_jump() {
        let mut rope = Rope::new(4);
        rope.simulate(&Movement {
            direction: Direction::Right,
            amount: 3,
        });
        rope.simulate(&Movement {
            direction: Direction::Down,
            amount: 2,
        });

        assert_eq!(rope.get_head_position(), (3, 2));
        assert_eq!(rope.get_tail_position(), (1, 1));
        assert_eq!(rope.count_total_tail_visited_locations(), 2);
    }
}
