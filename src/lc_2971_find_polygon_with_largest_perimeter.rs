use crate::Solution;

impl Solution {
    pub fn largest_perimeter(mut nums: Vec<i32>) -> i64 {
        // nums[1,  12,  1,  2,  5, 50, 3]
        // nums[50, 12,  5,  3,  2,  1, 1]
        // nums[74, 24, 12,  7,  4,  2, 1]
        nums.sort_by(|a, b| b.cmp(a));
        let nums = nums.iter().map(|&x| x.into()).collect::<Vec<i64>>();
        let mut sum_value: i64 = nums.iter().sum();
        let mut ans = -1;

        nums.iter().for_each(|&e| {
            sum_value -= e;
            if sum_value > e {
                ans = ans.max(sum_value + e);
            }
        });

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(Solution::largest_perimeter(vec![5, 5, 5]), 15);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(Solution::largest_perimeter(vec![1, 12, 1, 2, 5, 50, 3]), 12);
    }

    #[test]
    fn test_case_3() {
        assert_eq!(Solution::largest_perimeter(vec![5, 5, 50]), -1);
    }
}
