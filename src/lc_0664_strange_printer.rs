use crate::Solution;

impl Solution {
    pub fn strange_printer(s: String) -> i32 {
        let n = s.len();
        let mut dp = vec![vec![i32::MAX; n]; n];

        for i in (0..n).rev() {
            dp[i][i] = 1;
            for j in i + 1..n {
                if s.chars().nth(i) == s.chars().nth(j) {
                    dp[i][j] = dp[i][j - 1];
                } else {
                    for k in i..j {
                        dp[i][j] = dp[i][j].min(dp[i][k] + dp[k + 1][j]);
                    }
                }
            }
        }

        dp[0][n - 1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let s = "aaabbb".to_string();
        assert_eq!(Solution::strange_printer(s), 2);
    }

    #[test]
    fn test_case_2() {
        let s = "aba".to_string();
        assert_eq!(Solution::strange_printer(s), 2);
    }
}
