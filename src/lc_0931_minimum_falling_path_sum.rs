use crate::Solution;

impl Solution {
    pub fn min_falling_path_sum(matrix: Vec<Vec<i32>>) -> i32 {
        if matrix.is_empty() {
            return 0;
        }
        let mut dp = vec![vec![0; matrix[0].len()]; matrix.len()];
        for (r, row) in matrix.iter().enumerate() {
            for (c, col) in row.iter().enumerate() {
                if r == 0 {
                    dp[r][c] = *col;
                    continue;
                }
                dp[r][c] = *col
                    + dp[(r - 1).max(0)][c]
                        .min(dp[(r - 1).max(0)][(c + 1).min(matrix[0].len() - 1)])
                        .min(dp[(r - 1).max(0)][c.saturating_sub(1)]);
            }
        }
        *dp.last().unwrap().iter().min().unwrap_or(&0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_should_be_13() {
        let matrix = vec![vec![2, 1, 3], vec![6, 5, 4], vec![7, 8, 9]];
        assert_eq!(13, Solution::min_falling_path_sum(matrix));
    }

    #[test]
    fn test_should_be_neg_59() {
        let matrix = vec![vec![-19, 57], vec![-40, -5]];
        assert_eq!(-59, Solution::min_falling_path_sum(matrix));
    }
}

