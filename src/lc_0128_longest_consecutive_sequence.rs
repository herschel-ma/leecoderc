use crate::Solution;
use std::collections::HashSet;

impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let num_set: HashSet<i32> = nums.into_iter().collect();
        let mut ans = 0;

        for num in num_set.iter() {
            // this is the start number in a range
            if !num_set.contains(&(num - 1)) {
                let mut count = 1;
                let mut cur_nums = *num;

                while num_set.contains(&(cur_nums + 1)) {
                    cur_nums += 1;
                    count += 1;
                }
                ans = ans.max(count);
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
        let nums = vec![100, 4, 200, 1, 3, 2];
        assert_eq!(Solution::longest_consecutive(nums), 4);
    }

    #[test]
    fn test_case_2() {
        let nums = vec![0, 3, 7, 2, 5, 8, 4, 6, 0, 1];
        assert_eq!(Solution::longest_consecutive(nums), 9);
    }
}

