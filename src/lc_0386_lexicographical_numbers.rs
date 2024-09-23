use crate::Solution;

impl Solution {
    pub fn lexical_order(n: i32) -> Vec<i32> {
        let mut ans = Vec::new();
        let mut i = 1;
        for _ in 1..=n {
            ans.push(i);
            if i * 10 <= n {
                i *= 10;
            } else {
                while i % 10 == 9 || i >= n {
                    i /= 10;
                }
                i += 1;
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let output = vec![1, 10, 11, 12, 13, 2, 3, 4, 5, 6, 7, 8, 9];
        assert_eq!(Solution::lexical_order(13), output);
    }

    #[test]
    fn test_case_2() {
        let output = vec![1, 2];
        assert_eq!(Solution::lexical_order(2), output);
    }
}
