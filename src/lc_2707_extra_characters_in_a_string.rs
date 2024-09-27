use std::collections::HashSet;

use crate::Solution;

impl Solution {
    pub fn min_extra_char(s: String, dictionary: Vec<String>) -> i32 {
        let max_val = s.len() + 1;
        let mut dp = vec![max_val; s.len() + 1];
        dp[0] = 0;

        let dictionary: HashSet<String> = HashSet::from_iter(dictionary);

        for i in 1..=s.len() {
            dp[i] = dp[i - 1] + 1;
            for j in 1..=i {
                if dictionary.contains(&s[i - j..i]) {
                    dp[i] = std::cmp::min(dp[i - j], dp[i]);
                }
            }
        }
        *dp.last().unwrap() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let s = String::from("leetpcode");
        let dictionary = vec![
            String::from("leet"),
            String::from("code"),
            String::from("leetcode"),
        ];
        assert_eq!(Solution::min_extra_char(s, dictionary), 1);
    }

    #[test]
    fn test_case_2() {
        let s = String::from("sayhelloworld");
        let dictionary = vec![String::from("hello"), String::from("world")];
        assert_eq!(Solution::min_extra_char(s, dictionary), 3);
    }
}
