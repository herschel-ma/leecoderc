use crate::Solution;

impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        if x == 0 {
            return -0;
        }
        if x > 0 && x <= 3 {
            return 1;
        }
        let mut l = 2;
        let mut r = std::cmp::min(x / 2, 46340);
        while l < r {
            let mid = (l + r + 1) >> 1;
            if mid * mid > x {
                r = mid - 1;
            } else {
                l = mid;
            }
        }
        l
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let x = 4;
        assert_eq!(Solution::my_sqrt(x), 2);
    }
    #[test]
    fn test_case_2() {
        let x = 8;
        assert_eq!(Solution::my_sqrt(x), 2);
    }
}

