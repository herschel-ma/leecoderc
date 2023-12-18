use crate::Solution;

impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let mut mx = 0_i32;
        for (i, v) in nums.iter().enumerate() {
            if mx < i as i32 {
                return false;
            }
            mx = mx.max(i as i32 + v);
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_case_1() {
        let nums = vec![2, 3, 1, 1, 4];
        assert!(Solution::can_jump(nums));
    }

    #[test]
    fn test_case_2() {
        let nums = vec![3, 2, 1, 0, 4];
        assert!(!Solution::can_jump(nums));
    }
}
