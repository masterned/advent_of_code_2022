use std::collections::{HashSet, VecDeque};

use crate::altitude::Altitude;

#[derive(Debug)]
pub struct Atlas {
    altitudes: Vec<Vec<Altitude>>,
    pub start: Option<(usize, usize)>,
    pub end: Option<(usize, usize)>,
}

impl From<Vec<&str>> for Atlas {
    fn from(value: Vec<&str>) -> Self {
        Atlas::from(
            value
                .iter()
                .map(|row| row.chars().collect())
                .collect::<Vec<Vec<char>>>(),
        )
    }
}

impl From<Vec<Vec<char>>> for Atlas {
    fn from(value: Vec<Vec<char>>) -> Self {
        let mut start = None;
        let mut end = None;

        let altitudes = value
            .iter()
            .enumerate()
            .map(|(y, r)| {
                r.iter()
                    .enumerate()
                    .map(|(x, &c)| {
                        let altitude = Altitude::from(c);
                        match altitude {
                            Altitude::Start => start = Some((x, y)),
                            Altitude::End => end = Some((x, y)),
                            _ => (),
                        }
                        altitude
                    })
                    .collect()
            })
            .collect();

        Self {
            altitudes,
            start,
            end,
        }
    }
}

impl Atlas {
    fn get_neighbors_of(&self, (x, y): (usize, usize)) -> Vec<(usize, usize)> {
        let mut neighbors = vec![];

        if let Some(&altitude) = self.altitudes.get(y).and_then(|row| row.get(x)) {
            if y > 0 {
                if let Some(north) = self.altitudes.get(y - 1).and_then(|row| row.get(x)) {
                    if altitude.can_reach(north) {
                        neighbors.push((x, y - 1));
                    }
                }
            }
            if let Some(east) = self.altitudes.get(y).and_then(|row| row.get(x + 1)) {
                if altitude.can_reach(east) {
                    neighbors.push((x + 1, y));
                }
            }
            if let Some(south) = self.altitudes.get(y + 1).and_then(|row| row.get(x)) {
                if altitude.can_reach(south) {
                    neighbors.push((x, y + 1));
                }
            }
            if x > 0 {
                if let Some(west) = self.altitudes.get(y).and_then(|row| row.get(x - 1)) {
                    if altitude.can_reach(west) {
                        neighbors.push((x - 1, y));
                    }
                }
            }
        }

        neighbors
    }

    fn contains_point(&self, (x, y): (usize, usize)) -> bool {
        self.altitudes
            .get(y)
            .is_some_and(|row| row.get(x).is_some())
    }

    fn count_total_points(&self) -> usize {
        self.altitudes.iter().map(|row| row.len()).sum()
    }

    pub fn count_fewest_steps(&self, start: (usize, usize), end: (usize, usize)) -> Option<usize> {
        if !self.contains_point(start) || !self.contains_point(end) {
            return None;
        }

        let mut steps = 0;

        let mut visited: HashSet<(usize, usize)> = HashSet::new();
        let mut next_points: VecDeque<(usize, usize)> = VecDeque::from(vec![start]);

        let total_points = self.count_total_points();

        while visited.len() < total_points {
            let current_points: Vec<(usize, usize)> = next_points.drain(..).collect();

            for current_point in current_points {
                if current_point == end {
                    return Some(steps);
                }

                let neighbors = self.get_neighbors_of(current_point);

                for neighbor in neighbors {
                    if !visited.contains(&neighbor) && !next_points.contains(&neighbor) {
                        next_points.push_back(neighbor);
                    }
                }

                visited.insert(current_point);
            }

            steps += 1;
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod from {
        use super::*;

        #[test]
        fn _should_set_start_if_present() {
            let atlas = Atlas::from(vec![vec!['a', 'S']]);
            assert_eq!(atlas.start, Some((1, 0)));
        }

        #[test]
        fn _should_set_start_to_none_if_missing() {
            let atlas = Atlas::from(vec![vec!['a', 'b', 'c']]);
            assert_eq!(atlas.start, None);
        }

        #[test]
        fn _should_set_end_if_present() {
            let atlas = Atlas::from(vec![vec!['a', 'b', 'E']]);
            assert_eq!(atlas.end, Some((2, 0)));
        }

        #[test]
        fn _should_set_end_to_none_if_missing() {
            let atlas = Atlas::from(vec![vec!['a', 'b', 'c']]);
            assert_eq!(atlas.end, None);
        }
    }

    mod get_neighbors_of {
        use super::*;

        #[test]
        fn _should_return_empty_vec_on_no_neighbors() {
            let matrix = Atlas::from(vec![vec!['S', 'E']]);
            assert_eq!(matrix.get_neighbors_of((0, 0)), vec![]);
        }

        #[test]
        fn _should_only_return_reachable_neighbors() {
            let matrix = Atlas::from(vec![vec!['S', 'E'], vec!['a']]);
            assert_eq!(matrix.get_neighbors_of((0, 0)), vec![(0, 1)]);
        }

        #[test]
        fn _should_return_neighbors_in_clockwise_order_from_top() {
            let matrix = Atlas::from(vec![
                vec!['E', 'a', 'z'],
                vec!['a', 'S', 'a'],
                vec!['z', 'a', 'z'],
            ]);

            assert_eq!(
                matrix.get_neighbors_of((1, 1)),
                vec![(1, 0), (2, 1), (1, 2), (0, 1)]
            );
        }

        #[test]
        fn _neighbors_of_same_height_are_reachable() {
            let matrix = Atlas::from(vec![vec!['S', 'a', 'a'], vec!['E', 'a']]);
            assert_eq!(
                matrix.get_neighbors_of((1, 0)),
                vec![(2, 0), (1, 1), (0, 0)]
            );
        }
    }

    mod contains_point {
        use super::*;

        #[test]
        fn _should_return_true_if_atlas_contains_point() {
            let atlas = Atlas::from(vec![
                vec!['a', 'b', 'c'],
                vec!['d', 'e', 'f'],
                vec!['g', 'h', 'i'],
            ]);

            assert!(atlas.contains_point((0, 0)));
            assert!(atlas.contains_point((1, 1)));
            assert!(atlas.contains_point((2, 2)));
        }

        #[test]
        fn _should_return_false_if_atlas_does_not_contain_point() {
            let atlas = Atlas::from(vec![
                vec!['a', 'b', 'c'],
                vec!['d', 'e', 'f'],
                vec!['g', 'h', 'i'],
            ]);

            assert!(!atlas.contains_point((3, 3)));
            assert!(!atlas.contains_point((0, 7)));
        }
    }

    mod count_total_points {
        use super::*;

        #[test]
        fn _should_return_0_on_empty_atlas() {
            let atlas = Atlas::from(vec![vec![]]);
            assert_eq!(atlas.count_total_points(), 0);
        }

        #[test]
        fn _should_return_total_number_points_in_atlas() {
            let atlas = Atlas::from(vec!["spencer"]);
            assert_eq!(atlas.count_total_points(), 7);

            let atlas = Atlas::from(vec!["abc", "def", "ghi"]);
            assert_eq!(atlas.count_total_points(), 9);
        }
    }

    mod count_fewest_steps {
        use super::*;

        #[test]
        fn _should_return_some_0_when_start_and_end_are_same() {
            let atlas = Atlas::from(vec![vec!['S', 'E']]);
            assert_eq!(atlas.count_fewest_steps((0, 0), (0, 0)), Some(0));
        }

        #[test]
        fn _should_return_none_if_start_not_in_atlas() {
            let atlas = Atlas::from(vec![vec!['a']]);
            assert_eq!(atlas.count_fewest_steps((1, 1), (0, 0)), None);
        }

        #[test]
        fn _should_return_none_if_end_not_in_atlas() {
            let atlas = Atlas::from(vec![vec!['a']]);
            assert_eq!(atlas.count_fewest_steps((0, 0), (1, 2)), None);
        }

        #[test]
        fn _should_return_some_1_if_end_next_to_and_reachable_from_start() {
            let atlas = Atlas::from(vec![vec!['a', 'b']]);
            assert_eq!(atlas.count_fewest_steps((0, 0), (1, 0)), Some(1));
        }

        #[test]
        fn _should_return_fewest_steps_from_example() {
            let example = Atlas::from(vec![
                "Sabqponm", "abcryxxl", "accszExk", "acctuvwj", "abdefghi",
            ]);

            assert_eq!(example.start, Some((0, 0)));
            assert_eq!(example.end, Some((5, 2)));

            assert_eq!(example.count_fewest_steps((0, 0), (5, 2)), Some(31));
        }

        #[test]
        fn _should_find_fewest_steps_with_dead_ends() {
            let atlas = Atlas::from(vec!["aaaaaS", "bcdxxa", "xxxxxa"]);

            assert_eq!(atlas.count_fewest_steps((5, 0), (2, 1)), Some(8));
        }

        #[test]
        fn _should_return_shortest_when_presented_multiple_paths() {
            let atlas = Atlas::from(vec!["Saaaa", "axedb", "bcdcc"]);

            assert_eq!(atlas.count_fewest_steps((0, 0), (2, 1)), Some(5));
        }

        fn _should_return_none_if_no_route() {
            let atlas = Atlas::from(vec!["Sab", "gEc", "fed"]);

            assert_eq!(atlas.start, Some((0, 0)));
            assert_eq!(atlas.end, Some((1, 1)));

            assert_eq!(
                atlas.count_fewest_steps(atlas.start.unwrap(), atlas.end.unwrap()),
                None
            );
        }
    }
}
