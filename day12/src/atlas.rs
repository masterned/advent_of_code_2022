use std::{
    collections::{hash_map::Entry, HashMap, VecDeque},
    error::Error,
    fmt::Display,
    str::FromStr,
};

use crate::altitude::Altitude;

#[derive(Debug, PartialEq, Eq)]
pub enum CountFewestStepsError {
    CannotFindStart,
    CannotFindEnd,
    NoRouteFound,
}

impl Display for CountFewestStepsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CountFewestStepsError::CannotFindStart => write!(f, "Unable to find start."),
            CountFewestStepsError::CannotFindEnd => write!(f, "Unable to find end."),
            CountFewestStepsError::NoRouteFound => {
                write!(f, "Unable to find route from start to end.")
            }
        }
    }
}

impl Error for CountFewestStepsError {}

#[derive(Debug)]
pub struct Atlas {
    altitudes: Vec<Vec<Option<Altitude>>>,
    start: Option<(usize, usize)>,
    end: Option<(usize, usize)>,
}

impl FromStr for Atlas {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.lines().collect::<Vec<&str>>().into())
    }
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
                        let altitude = Altitude::try_from(c).ok();
                        match altitude {
                            Some(Altitude::Start) => start = Some((x, y)),
                            Some(Altitude::End) => end = Some((x, y)),
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
    fn get_altitude(&self, (x, y): (usize, usize)) -> Option<Altitude> {
        *self.altitudes.get(y).and_then(|row| row.get(x))?
    }

    fn get_neighbors_of(&self, (x, y): (usize, usize)) -> Vec<(usize, usize)> {
        let mut neighbors = vec![];

        if let Some(altitude) = self.get_altitude((x, y)) {
            if y > 0 {
                if let Some(north) = self.get_altitude((x, y - 1)) {
                    if altitude.can_reach(&north) {
                        neighbors.push((x, y - 1));
                    }
                }
            }
            if let Some(east) = self.get_altitude((x + 1, y)) {
                if altitude.can_reach(&east) {
                    neighbors.push((x + 1, y));
                }
            }
            if let Some(south) = self.get_altitude((x, y + 1)) {
                if altitude.can_reach(&south) {
                    neighbors.push((x, y + 1));
                }
            }
            if x > 0 {
                if let Some(west) = self.get_altitude((x - 1, y)) {
                    if altitude.can_reach(&west) {
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

    pub fn count_fewest_steps_from_start_to_end(&self) -> Result<usize, CountFewestStepsError> {
        let start = self.start.ok_or(CountFewestStepsError::CannotFindStart)?;
        let end = self.end.ok_or(CountFewestStepsError::CannotFindEnd)?;
        let result = self.count_fewest_steps(start, end)?;
        Ok(result)
    }

    fn create_parent_map(
        &self,
        start: (usize, usize),
        end: (usize, usize),
    ) -> HashMap<(usize, usize), Option<(usize, usize)>> {
        let mut parents = HashMap::from([(start, None)]);

        let mut next_points = VecDeque::from(vec![start]);

        while !next_points.is_empty() {
            if let Some(current_point) = next_points.pop_front() {
                let children = self.get_neighbors_of(current_point);

                for child in children {
                    if let Entry::Vacant(entry) = parents.entry(child) {
                        entry.insert(Some(current_point));

                        if child == end {
                            next_points.clear();
                            break;
                        }

                        next_points.push_back(child);
                    }
                }
            }
        }

        parents
    }

    fn count_fewest_steps(
        &self,
        start: (usize, usize),
        end: (usize, usize),
    ) -> Result<usize, CountFewestStepsError> {
        if !self.contains_point(start) {
            return Err(CountFewestStepsError::CannotFindStart);
        }
        if !self.contains_point(end) {
            return Err(CountFewestStepsError::CannotFindEnd);
        }
        if start == end {
            return Ok(0);
        }

        let parents = self.create_parent_map(start, end);

        let mut step = 0;
        let mut current_point = parents.get(&end);

        loop {
            match current_point {
                Some(&None) => return Ok(step),
                Some(&Some(point)) => {
                    current_point = parents.get(&point);
                    step += 1;
                }
                None => return Err(CountFewestStepsError::NoRouteFound),
            }
        }
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

    mod create_parent_map {
        use super::*;

        #[test]
        fn _should_set_root_parent_to_some_none() {
            let atlas = Atlas::from(vec!["S"]);
            let parent_map = atlas.create_parent_map((0, 0), (0, 0));
            let root_parent = parent_map.get(&(0, 0));

            assert_eq!(root_parent, Some(&None));
        }

        #[test]
        fn _should_set_parent_of_point() {
            let atlas = Atlas::from(vec!["Sa"]);
            let parent_map = atlas.create_parent_map((0, 0), (1, 0));

            assert_eq!(parent_map.get(&(1, 0)), Some(&Some((0, 0))));
        }

        #[test]
        fn _should_set_parent_to_first_found_parent() {
            let atlas = Atlas::from(vec!["Sab", "adc", "aef", "bce", "cde"]);
            let parent_map = atlas.create_parent_map((0, 0), (2, 2));

            assert_eq!(parent_map.get(&(1, 1)), Some(&Some((2, 1))));
            assert_eq!(parent_map.get(&(1, 2)), Some(&Some((1, 1))));
            assert_eq!(parent_map.get(&(2, 2)), Some(&Some((1, 2))));
        }
    }

    mod count_fewest_steps {
        use super::*;

        #[test]
        fn _should_return_some_0_when_start_and_end_are_same() {
            let atlas = Atlas::from(vec![vec!['S', 'E']]);
            assert_eq!(atlas.count_fewest_steps((0, 0), (0, 0)), Ok(0));
        }

        #[test]
        fn _should_return_error_if_start_not_in_atlas() {
            let atlas = Atlas::from(vec![vec!['a']]);
            assert_eq!(
                atlas.count_fewest_steps((1, 1), (0, 0)),
                Err(CountFewestStepsError::CannotFindStart)
            );
        }

        #[test]
        fn _should_return_error_if_end_not_in_atlas() {
            let atlas = Atlas::from(vec![vec!['a']]);
            assert_eq!(
                atlas.count_fewest_steps((0, 0), (1, 2)),
                Err(CountFewestStepsError::CannotFindEnd)
            );
        }

        #[test]
        fn _should_return_some_1_if_end_next_to_and_reachable_from_start() {
            let atlas = Atlas::from(vec![vec!['a', 'b']]);
            assert_eq!(atlas.count_fewest_steps((0, 0), (1, 0)), Ok(1));
        }

        #[test]
        fn _should_return_fewest_steps_from_example() {
            let example = Atlas::from(vec![
                "Sabqponm", "abcryxxl", "accszExk", "acctuvwj", "abdefghi",
            ]);

            assert_eq!(example.start, Some((0, 0)));
            assert_eq!(example.end, Some((5, 2)));

            assert_eq!(example.count_fewest_steps((0, 0), (5, 2)), Ok(31));
        }

        #[test]
        fn _should_find_fewest_steps_with_dead_ends() {
            let atlas = Atlas::from(vec!["aaaaaS", "bcdxxa", "xxxxxa"]);

            assert_eq!(atlas.count_fewest_steps((5, 0), (2, 1)), Ok(8));
        }

        #[test]
        fn _should_return_shortest_when_presented_multiple_paths() {
            let atlas = Atlas::from(vec!["Saaaa", "axedb", "bcdcc"]);

            assert_eq!(atlas.count_fewest_steps((0, 0), (2, 1)), Ok(5));
        }

        #[test]
        fn _should_return_error_if_no_route() {
            let atlas = Atlas::from(vec!["Sab", "gEc", "fed"]);

            assert_eq!(atlas.start, Some((0, 0)));
            assert_eq!(atlas.end, Some((1, 1)));

            assert_eq!(
                atlas.count_fewest_steps(atlas.start.unwrap(), atlas.end.unwrap()),
                Err(CountFewestStepsError::NoRouteFound)
            );
        }
    }
}
