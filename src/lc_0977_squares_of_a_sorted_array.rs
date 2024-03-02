use crate::Solution;

impl Solution {
    pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
        if nums.len() == 1 {
            return vec![i32::pow(nums[0], 2)];
        }

        let mut location = nums.len();
        let mut left = 0;
        let mut right = nums.len() - 1;
        let mut output = vec![0; location];
        let mut computed_left = i32::pow(nums[left], 2);
        let mut computed_right = i32::pow(nums[right], 2);

        while left != right {
            location -= 1;
            if computed_left > computed_right {
                output[location] = computed_left;
                left += 1;
                computed_left = i32::pow(nums[left], 2);
            } else {
                output[location] = computed_right;
                right -= 1;
                computed_right = i32::pow(nums[right], 2);
            }
        }
        output[0] = computed_left;

        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tests_case_1() {
        let nums = vec![-4, -1, 0, 3, 10];
        let output = vec![0, 1, 9, 16, 100];
        assert_eq!(Solution::sorted_squares(nums), output);
    }

    #[test]
    fn tests_case_2() {
        let nums = vec![-7, -3, 2, 3, 11];
        let output = vec![4, 9, 9, 49, 121];
        assert_eq!(Solution::sorted_squares(nums), output);
    }
}
