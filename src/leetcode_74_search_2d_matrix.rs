struct Solution;

impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        search_matrix(&matrix, target)
    }
}

fn search_matrix(matrix: &Vec<Vec<i32>>, target: i32) -> bool {
    let next_row = matrix.partition_point(|v| v[0] <= target);
    if next_row == 0 {
        false
    } else {
        matrix[next_row - 1].binary_search(&target).is_ok()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn matrix() -> Vec<Vec<i32>> {
        vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]]
    }

    #[test]
    fn search_matrix_true() {
        let m = matrix();
        for row in &m {
            for &n in row {
                assert_eq!(search_matrix(&m, n), true)
            }
        }
    }

    #[test]
    fn example2() {
        let m = matrix();
        let invalid_targets = [0, 2, 4, 6, 13, 18, 21, 25, 33, 59, 61, 62];
        for n in invalid_targets {
            assert_eq!(search_matrix(&m, n), false)
        }
    }
}
