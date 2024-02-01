use crate::Solution;

impl Solution {
    pub fn divide_array(mut nums: Vec<i32>, k: i32) -> Vec<Vec<i32>> {
        nums.sort_unstable();
        let mut ans = vec![];
        for arr in nums.chunks(3).map(Vec::from) {
            if arr[2] - arr[0] > k {
                return vec![];
            }
            ans.push(arr)
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let nums = vec![1, 3, 4, 8, 7, 9, 3, 5, 1];
        let output = vec![vec![1, 1, 3], vec![3, 4, 5], vec![7, 8, 9]];
        assert_eq!(Solution::divide_array(nums, 2), output);
    }
    #[test]
    fn test_case_2() {
        let nums = vec![1, 3, 3, 2, 7, 3];
        let output: Vec<Vec<i32>> = Vec::new();
        assert_eq!(Solution::divide_array(nums, 3), output);
    }
}
