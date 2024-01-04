use crate::Solution;

impl Solution {
    pub fn min_operations(nums: Vec<i32>, x: i32) -> i32 {
        // sliding window and prefix sum
        let n = nums.iter().sum::<i32>() - x;
        let mut l = 0;
        let mut res: i32 = -1;
        let mut cur_sum = 0;

        for r in 0..nums.len() {
            cur_sum += nums[r];
            while l <= r && cur_sum > n {
                cur_sum -= nums[l];
                l += 1;
            }

            if cur_sum == n {
                res = res.max((r - l + 1) as i32)
            }
        }

        if res == -1 {
            -1
        } else {
            nums.len() as i32 - res
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let nums = Vec::from([1, 1, 4, 2, 3]);
        let x = 5;
        assert_eq!(Solution::min_operations(nums, x), 2);
    }

    #[test]
    fn test_case_2() {
        let nums = Vec::from([5, 6, 7, 8, 9]);
        let x = 4;
        assert_eq!(Solution::min_operations(nums, x), -1);
    }

    #[test]
    fn test_case_3() {
        let nums = Vec::from([3, 2, 20, 1, 1, 3]);
        let x = 10;
        assert_eq!(Solution::min_operations(nums, x), 5);
    }
}

