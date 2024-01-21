use std::iter;

use crate::Solution;

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let (mut pre1, mut pre2) = (0, 0);
        for num in nums.iter() {
            let current = pre1.max(num + pre2);
            pre2 = pre1;
            pre1 = current;
        }
        pre1
    }

    pub fn rob_2(nums: Vec<i32>) -> i32 {
        let mut dp = vec![0; nums.len() + 2];
        for (i, n) in nums.into_iter().enumerate() {
            dp[i + 1] = dp[i + 1].max(dp[i]);
            dp[i + 2] = dp[i + 2].max(dp[i] + n);
        }
        dp.into_iter().max().unwrap()
    }

    pub fn rob_3(nums: Vec<i32>) -> i32 {
        let money_in_house = nums;
        money_in_house
            .into_iter()
            .chain(iter::once(0))
            .fold((0, 0), |(can_rob, unrob), money| {
                let rob_this_house = can_rob + money;
                let can_rob_next = can_rob.max(unrob);
                (can_rob_next, rob_this_house)
            })
            .0
    }

    pub fn rob_4(nums: Vec<i32>) -> i32 {
        let mut dp = vec![0, 0, 0];
        for (i, num) in nums.into_iter().enumerate() {
            dp[(i + 2) % 3] = dp[(i + 2) % 3].max(dp[i % 3] + num);
            dp[(i + 1) % 3] = dp[(i + 1) % 3].max(dp[i % 3]);
        }
        dp.into_iter().max().unwrap()
    }

    pub fn rob_5(nums: Vec<i32>) -> i32 {
        nums.into_iter()
            .fold((0, 0), |acc, n| (acc.1, acc.1.max(acc.0 + n)))
            .1
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    type Rob = fn(nums: Vec<i32>) -> i32;

    #[test]
    fn test_case_1() {
        let rob: Rob = Solution::rob_5;
        assert_eq!(rob(vec![1, 2, 3, 1]), 4);
    }

    #[test]
    fn test_case_2() {
        let rob: Rob = Solution::rob_5;
        assert_eq!(rob(vec![2, 7, 9, 3, 1]), 12);
    }
}

