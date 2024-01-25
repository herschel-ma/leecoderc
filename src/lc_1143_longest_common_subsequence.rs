use crate::Solution;

impl Solution {
    pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
        let mut dp: Vec<Vec<i32>> = vec![vec![0; text2.len() + 1]; text1.len() + 1];
        for (i, c1) in text1.chars().enumerate() {
            for (j, c2) in text2.chars().enumerate() {
                dp[i + 1][j + 1] = if c1 == c2 {
                    dp[i][j] + 1
                } else {
                    dp[i + 1][j].max(dp[i][j + 1])
                };
            }
        }
        dp[text1.len()][text2.len()]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let text1 = "abcde".to_string();
        let text2 = "ace".to_string();
        assert_eq!(Solution::longest_common_subsequence(text1, text2), 3);
    }

    #[test]
    fn test_case_2() {
        let text1 = "abc".to_string();
        let text2 = "abc".to_string();
        assert_eq!(Solution::longest_common_subsequence(text1, text2), 3);
    }

    #[test]
    fn test_case_3() {
        let text1 = "abc".to_string();
        let text2 = "def".to_string();
        assert_eq!(Solution::longest_common_subsequence(text1, text2), 0);
    }
}
