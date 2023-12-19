use crate::Solution;

impl Solution {
    pub fn min_distance(word1: String, word2: String) -> i32 {
        // a b d
        // a c d
        // 1. a(word1[0]) == a(word2[0]) =>  b d  | c d   => if word1[i] = word2[j] => f[i][j] = f[i+1][j+1]
        // b d
        // c d
        // insert=> c b d | c d => i .. j + 1 .. => f[i][j+1] + 1(insert)
        // remove=> d | c d => i + 1 .. j .. => f[i+1][j] + 1(remove)
        // replace => c d | c d  => i + 1 .. j + 1 .. => f[i+1][j+1] + 1(replace)
        let m = word1.len();
        let n = word2.len();
        let mut dp = vec![vec![0; n + 1]; m + 1];
        let word1 = word1.into_bytes();
        let word2 = word2.into_bytes();
        for i in 0..n {
            dp[m][i] = n - i;
        }
        for (j, item) in dp.iter_mut().enumerate() {
            item[n] = m - j;
        }

        for i in (0..m).rev() {
            for j in (0..n).rev() {
                if word1[i] == word2[j] {
                    dp[i][j] = dp[i + 1][j + 1];
                } else {
                    dp[i][j] = 1 + (dp[i][j + 1].min(dp[i + 1][j]).min(dp[i + 1][j + 1]));
                }
            }
        }
        dp[0][0] as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let word1 = "horse".to_string();
        let word2 = "ros".to_string();
        assert_eq!(Solution::min_distance(word1, word2), 3);
    }

    #[test]
    fn test_case_2() {
        let word1 = "intention".to_string();
        let word2 = "execution".to_string();
        assert_eq!(Solution::min_distance(word1, word2), 5);
    }

    #[test]
    fn test_case_3() {
        let word1 = "".to_string();
        let word2 = "".to_string();
        assert_eq!(Solution::min_distance(word1, word2), 0);
    }
}
