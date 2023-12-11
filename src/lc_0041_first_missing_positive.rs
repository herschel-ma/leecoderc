pub fn first_missing_positive(nums: Vec<i32>) -> i32 {
    let mut nums = nums;
    let n = nums.len();
    // step 1: Move all positive numbers to the correct position
    for i in 0..n {
        let mut num = nums[i];
        while num > 0 && num <= n as i32 && nums[num as usize - 1] != num {
            std::mem::swap(&mut nums[num as usize - 1], &mut num);
        }
    }

    // step 2: Find the first missing positive number
    for (i, v) in nums.into_iter().enumerate() {
        if v != i as i32 + 1 {
            return i as i32 + 1;
        }
    }
    // If all positive numbers are present, the result is n + 1
    n as i32 + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let nums = Vec::from([1, 2, 0]);
        let output = 3;
        assert_eq!(first_missing_positive(nums), output);
    }

    #[test]
    fn test_case_2() {
        let nums = Vec::from([3, 4, -1, 1]);
        let output = 2;
        assert_eq!(first_missing_positive(nums), output);
    }

    #[test]
    fn test_case_3() {
        let nums = Vec::from([7, 8, 9, 11, 12]);
        let output = 1;
        assert_eq!(first_missing_positive(nums), output);
    }
}
