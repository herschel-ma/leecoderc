use crate::Solution;

impl Solution {
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        let n = digits.len();
        let mut digits = digits;

        for i in (0..n).rev() {
            digits[i] += 1;
            if digits[i] < 10 {
                return digits;
            }
            digits[i] %= 10;
        }
        digits.insert(0, 1);
        digits
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let digits = vec![1, 2, 3];
        let output = vec![1, 2, 4];
        assert_eq!(Solution::plus_one(digits), output);
    }

    #[test]
    fn test_case_2() {
        let digits = vec![4, 3, 2, 1];
        let output = vec![4, 3, 2, 2];
        assert_eq!(Solution::plus_one(digits), output);
    }

    #[test]
    fn test_case_3() {
        let digits = vec![9];
        let output = vec![1, 0];
        assert_eq!(Solution::plus_one(digits), output);
    }
}
