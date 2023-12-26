use crate::Solution;

impl Solution {
    pub fn num_decodings(s: String) -> i32 {
        if s.chars().nth(0) == Some('0') {
            return 0;
        }

        let mut dp = vec![0; s.len()];
        dp.insert(0, 1);

        for i in 1..=s.len() {
            if s.chars().nth(i - 1).unwrap().to_digit(10) != Some(0) {
                dp[i] = dp[i - 1];
            }
            if i > 1
                && s.chars().nth(i - 2).unwrap().to_digit(10) != Some(0)
                && s[i - 2..i].parse::<i32>().unwrap() <= 26
            {
                dp[i] += dp[i - 2]
            }
        }

        dp[s.len()]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let s = String::from("12");
        assert_eq!(Solution::num_decodings(s), 2);
    }

    #[test]
    fn test_case_2() {
        let s = String::from("226");
        assert_eq!(Solution::num_decodings(s), 3);
    }

    #[test]
    fn test_case_3() {
        let s = String::from("06");
        assert_eq!(Solution::num_decodings(s), 0);
    }
}
