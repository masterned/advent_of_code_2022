pub fn get_surrounding(row: &Vec<i32>, index: usize) -> Vec<Vec<i32>> {
    let mut surrounding = Vec::new();

    let before = row[0..index].to_vec();
    let after = row[index + 1..row.len()].to_vec();
    surrounding.push(before);
    surrounding.push(after);

    surrounding
}

fn transpose(matrix: &[Vec<i32>]) -> Vec<Vec<i32>> {
    if matrix.is_empty() {
        return matrix.to_vec();
    }

    let mut result = Vec::new();

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

pub fn create_neighbourhood(matrix: &[Vec<i32>]) -> Vec<(i32, Vec<Vec<i32>>)> {
    let t_matrix = transpose(matrix);
    let mut neighbourhood = Vec::new();

    for (row_index, row) in matrix.iter().enumerate() {
        for (index, item) in row.iter().enumerate() {
            let mut surrounding = get_surrounding(row, index);
            get_surrounding(&t_matrix[index], row_index)
                .iter()
                .for_each(|a| surrounding.push(a.clone()));

            neighbourhood.push((*item, surrounding));
        }
    }

    neighbourhood
}

pub fn is_visible(tree: i32, neighbours: &[Vec<i32>]) -> bool {
    neighbours.iter().any(|line| line.iter().all(|&t| t < tree))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn _only_tree_should_be_visible() {
        let neighbours: Vec<Vec<i32>> = vec![Vec::new(), Vec::new(), Vec::new(), Vec::new()];
        assert!(is_visible(1, &neighbours));
    }

    #[test]
    fn _tree_surrounded_by_larger_trees_should_not_be_visible() {
        let neighbours = vec![vec![2], vec![2], vec![2], vec![2]];
        assert!(!is_visible(1, &neighbours));
    }

    #[test]
    fn _tree_surrounded_by_trees_of_same_size_should_not_be_visible() {
        let neighbours = vec![vec![1], vec![1], vec![1], vec![1]];
        assert!(!is_visible(1, &neighbours));
    }

    #[test]
    fn _should_create_neighbourhood_of_square_forest() {
        let forest = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let expected_result = vec![
            (1, vec![Vec::new(), vec![2, 3], Vec::new(), vec![4, 7]]),
            (2, vec![vec![1], vec![3], Vec::new(), vec![5, 8]]),
            (3, vec![vec![1, 2], Vec::new(), Vec::new(), vec![6, 9]]),
            (4, vec![Vec::new(), vec![5, 6], vec![1], vec![7]]),
            (5, vec![vec![4], vec![6], vec![2], vec![8]]),
            (6, vec![vec![4, 5], Vec::new(), vec![3], vec![9]]),
            (7, vec![Vec::new(), vec![8, 9], vec![1, 4], Vec::new()]),
            (8, vec![vec![7], vec![9], vec![2, 5], Vec::new()]),
            (9, vec![vec![7, 8], Vec::new(), vec![3, 6], Vec::new()]),
        ];
        assert_eq!(create_neighbourhood(&forest), expected_result);
    }
}
