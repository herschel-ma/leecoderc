use crate::Solution;
impl Solution {
    pub fn longest_subarray(nums: Vec<i32>) -> i32 {
        let (len, _, _) = nums.iter().fold(
            (0, 0, 0),
            |(last_max_len, current_max_len, current_max_value), &num| match num {
                v if v > current_max_value => (1, 1, v),
                v if v < current_max_value => (last_max_len, 0, current_max_value),
                _ => (
                    core::cmp::max(last_max_len, current_max_len + 1),
                    current_max_len + 1,
                    current_max_value,
                ),
            },
        );
        len
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let nums = vec![1, 2, 3, 3, 2, 2];
        assert_eq!(Solution::longest_subarray(nums), 2);
    }

    #[test]
    fn test_case_2() {
        let nums = vec![1, 2, 3, 4];
        assert_eq!(Solution::longest_subarray(nums), 1);
    }
}
