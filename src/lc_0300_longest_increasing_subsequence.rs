use crate::Solution;

impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }

        let mut dp = vec![nums[0]];
        for num in nums.iter().skip(1) {
            if num > dp.last().unwrap() {
                dp.push(*num);
            } else {
                let index = match dp.binary_search(num) {
                    Ok(idx) => idx,
                    Err(idx) => idx,
                };
                dp[index] = *num;
            }
        }

        dp.len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let nums = vec![10, 9, 2, 5, 3, 7, 101, 18];
        assert_eq!(Solution::length_of_lis(nums), 4);
    }

    #[test]
    fn test_case_2() {
        let nums = vec![0, 1, 0, 3, 2, 3];
        assert_eq!(Solution::length_of_lis(nums), 4);
    }
    #[test]
    fn tsest_case_3() {
        let nums = vec![7, 7, 7, 7, 7, 7, 7];
        assert_eq!(Solution::length_of_lis(nums), 1);
    }
}

