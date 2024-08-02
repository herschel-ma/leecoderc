use crate::Solution;
impl Solution {
    pub fn min_swaps(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let total = nums.iter().filter(|&&x| x == 1).count();

        let mut max = 0;
        let mut lo = 0;
        let mut hi = 0;
        // current counts of ones within the window
        let mut win_ones = 0;
        // sliding window
        while hi < n * 2 {
            win_ones += nums[hi % n];
            if hi - lo + 1 > total {
                win_ones -= nums[lo % n];
                lo += 1;
            }
            max = max.max(win_ones);
            hi += 1;
        }
        total as i32 - max
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let nums = vec![0, 1, 0, 1, 1, 0, 0];
        assert_eq!(Solution::min_swaps(nums), 1);
    }

    #[test]
    fn test_case_2() {
        let nums = vec![0, 1, 1, 1, 0, 0, 1, 1, 0];
        assert_eq!(Solution::min_swaps(nums), 2);
    }

    #[test]
    fn test_case_3() {
        let nums = vec![1, 1, 0, 0, 1];
        assert_eq!(Solution::min_swaps(nums), 0);
    }
}
