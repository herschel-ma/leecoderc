use crate::Solution;

impl Solution {
    #[allow(clippy::mut_range_bound)]
    pub fn remove_duplicates_1(nums: &mut Vec<i32>) -> i32 {
        match nums.len() {
            0..=2 => nums.len() as i32,
            _ => {
                let mut ptr = 2;
                for i in ptr..nums.len() {
                    if nums[ptr - 2] != nums[i] {
                        nums[ptr] = nums[i];
                        ptr += 1;
                    }
                }
                ptr as i32
            }
        }
    }

    /// nearly my idea
    pub fn remove_duplicates_2(nums: &mut Vec<i32>) -> i32 {
        let mut index = 2;
        let mut n = nums.len();
        while index < n {
            if nums[index] == nums[index - 1] && nums[index] == nums[index - 2] {
                nums.remove(index);
                n -= 1;
            } else {
                index += 1;
            }
        }
        nums.len() as i32
    }

    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut k = 0;
        for i in 0..nums.len() {
            if k < 2 || nums[i] != nums[k - 2] {
                nums[k] = nums[i];
                k += 1;
            }
        }
        k as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let mut nums = vec![1, 1, 1, 2, 2, 3];
        let output = 5;
        assert_eq!(Solution::remove_duplicates(&mut nums), output);
    }

    #[test]
    fn test_case_2() {
        let mut nums = vec![0, 0, 1, 1, 1, 1, 2, 3, 3];
        let output = 7;
        assert_eq!(Solution::remove_duplicates(&mut nums), output);
    }
}
