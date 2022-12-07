use std::str::FromStr;

#[derive(Debug)]
pub struct Dock {
    stacks: Vec<Vec<char>>,
}

impl Dock {
    pub fn mv(&mut self, cmd: &Command) {
        for _ in 0..cmd.qty {
            let value = self.remove_crate(cmd.origin).unwrap();
            self.set_crate(cmd.dest, value);
        }
    }

    fn remove_crate(&mut self, stack_num: usize) -> Option<char> {
        let stack = self.stacks.get_mut(stack_num - 1)?;
        stack.pop()
    }

    fn set_crate(&mut self, stack_num: usize, value: char) {
        self.stacks
            .get_mut(stack_num - 1)
            .expect("Unable to find stack")
            .push(value);
    }

    pub fn get_top_crates(&self) -> String {
        let mut result = String::from("");
        for stack in self.stacks.clone() {
            if let Some(&c) = stack.last() {
                result.push(c);
            }
        }
        result
    }
}

pub struct ParseDockError {}

impl FromStr for Dock {
    type Err = ParseDockError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut spec: Vec<Vec<char>> = Vec::new();
        let lines = s.lines();
        for line in lines {
            let mut line_vec: Vec<char> = Vec::new();
            for c in line.chars() {
                line_vec.push(c);
            }
            spec.push(line_vec);
        }

        let spec = rotate_clockwise(&spec);
        let spec: Vec<&Vec<char>> = spec
            .iter()
            .filter(|row| row.first() != Some(&' '))
            .collect();
        let spec: Vec<Vec<&char>> = spec
            .iter()
            .map(|&row| row.iter().filter(|&item| (*item).is_alphabetic()).collect())
            .collect();
        // println!("{spec:?}");

        let mut stacks: Vec<Vec<char>> = Vec::new();
        for stack_spec in spec {
            let mut stack: Vec<char> = Vec::new();
            for item in stack_spec {
                stack.push(*item);
            }
            stacks.push(stack);
        }
        Ok(Dock { stacks })
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Command {
    pub qty: usize,
    pub origin: usize,
    pub dest: usize,
}

#[derive(Debug)]
pub struct ParseCommandError {}

impl FromStr for Command {
    type Err = ParseCommandError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tokens = s.split(' ');

        tokens.next();

        let qty_token = tokens.next().ok_or(Self::Err {})?;
        let qty = qty_token.parse().map_err(|_| Self::Err {})?;

        tokens.next();

        let origin_token = tokens.next().ok_or(Self::Err {})?;
        let origin = origin_token.parse().map_err(|_| Self::Err {})?;

        tokens.next();

        let dest_token = tokens.next().ok_or(Self::Err {})?;
        let dest = dest_token.parse().map_err(|_| Self::Err {})?;

        Ok(Self { qty, origin, dest })
    }
}

fn transpose(matrix: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    if matrix.is_empty() {
        return matrix.clone();
    }

    let mut result: Vec<Vec<char>> = Vec::new();

    for _ in 0..matrix[0].len() {
        result.push(Vec::new());
    }

    for row in matrix {
        for (index, item) in row.iter().enumerate() {
            result.get_mut(index).unwrap().push(*item);
        }
    }

    result
}

fn flip_x(matrix: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    if matrix.is_empty() {
        return Vec::new();
    }

    let mut result: Vec<Vec<char>> = matrix.clone();

    result.iter_mut().for_each(|row| row.reverse());

    result
}

fn rotate_clockwise(matrix: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    if matrix.is_empty() {
        return Vec::new();
    }

    flip_x(&transpose(matrix))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn _transposing_empty_matrix_should_return_empty_matrix() {
        let input: Vec<Vec<char>> = Vec::new();
        let result = transpose(&input);
        assert!(result.is_empty());
    }

    #[test]
    fn _transposing_singleton_matrix_should_return_singleton_matrix() {
        let input = vec![vec!['a']];
        let result = transpose(&input);
        assert_eq!(input, result);
    }

    #[test]
    fn _transpose_should_work_on_square_matrices() {
        let input = vec![vec!['a', 'b'], vec!['c', 'd']];
        let expected_output = vec![vec!['a', 'c'], vec!['b', 'd']];
        assert_eq!(transpose(&input), expected_output);

        let input = vec![
            vec!['a', 'b', 'c'],
            vec!['d', 'e', 'f'],
            vec!['g', 'h', 'i'],
        ];
        let expected_output = vec![
            vec!['a', 'd', 'g'],
            vec!['b', 'e', 'h'],
            vec!['c', 'f', 'i'],
        ];
        assert_eq!(transpose(&input), expected_output);
    }

    #[test]
    fn _transpose_should_work_on_asymmetric_matrices() {
        let input = vec![vec!['a', 'b']];
        let expected_output = vec![vec!['a'], vec!['b']];
        assert_eq!(transpose(&input), expected_output);

        let input = vec![
            vec!['a', 'b', 'c', 'd'],
            vec!['e', 'f', 'g', 'h'],
            vec!['i', 'j', 'k', 'l'],
        ];
        let expected_output = vec![
            vec!['a', 'e', 'i'],
            vec!['b', 'f', 'j'],
            vec!['c', 'g', 'k'],
            vec!['d', 'h', 'l'],
        ];
        assert_eq!(transpose(&input), expected_output);
    }
}
