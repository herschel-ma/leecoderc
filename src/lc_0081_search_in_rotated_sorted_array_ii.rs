use crate::Solution;
use std::cmp::Ordering;

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> bool {
        let mut l = 0;
        let mut r = nums.len() - 1;
        while l < r {
            let mid = (l + r) >> 1;
            match nums[mid].cmp(&nums[r]) {
                Ordering::Less => {
                    if nums[mid] < target && target <= nums[r] {
                        l = mid + 1;
                    } else {
                        r = mid;
                    }
                }
                Ordering::Greater => {
                    if nums[l] <= target && target <= nums[mid] {
                        r = mid;
                    } else {
                        l = mid + 1;
                    }
                }
                Ordering::Equal => r -= 1,
            }
        }

        nums[l] == target
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let nums = vec![2, 5, 6, 0, 0, 1, 2];
        let target = 0;
        assert!(Solution::search(nums, target));
    }

    #[test]
    fn test_case_2() {
        let nums = vec![2, 5, 6, 0, 0, 1, 2];
        let target = 3;
        assert!(!Solution::search(nums, target));
    }
}
