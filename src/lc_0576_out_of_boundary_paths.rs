use crate::Solution;
const MODE: i64 = 1_000_000_007;

impl Solution {
    pub fn find_paths(m: i32, n: i32, max_move: i32, start_row: i32, start_column: i32) -> i32 {
        fn f(
            i: usize,
            j: usize,
            moves: usize,
            m: usize,
            n: usize,
            dp: &mut Vec<Vec<Vec<i64>>>,
        ) -> i64 {
            if i < 1 || i > m || j < 1 || j > n {
                return 1;
            }
            if moves < 1 {
                return 0;
            }
            if dp[i][j][moves] != -1 {
                return dp[i][j][moves];
            }
            dp[i][j][moves] = (f(i + 1, j, moves - 1, m, n, dp)
                + f(i - 1, j, moves - 1, m, n, dp)
                + f(i, j + 1, moves - 1, m, n, dp)
                + f(i, j - 1, moves - 1, m, n, dp))
                % MODE;
            dp[i][j][moves]
        }

        let mut dp = vec![vec![vec![-1i64; 52]; 52]; 52];
        f(
            start_row as usize + 1,
            start_column as usize + 1,
            max_move as usize,
            m as usize,
            n as usize,
            &mut dp,
        ) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(Solution::find_paths(2, 2, 2, 0, 0), 6);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(Solution::find_paths(1, 3, 3, 0, 1), 12);
    }
}
