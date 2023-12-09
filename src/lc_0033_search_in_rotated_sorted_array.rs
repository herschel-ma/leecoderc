pub fn search_other(nums: Vec<i32>, target: i32) -> i32 {
    let n = nums.len();
    let mut left = 0;
    let mut right = n - 1;
    while left < right {
        let mid = (left + right) >> 1;
        if nums[0] <= nums[mid] {
            if nums[0] <= target && target <= nums[mid] {
                right = mid;
            } else {
                left = mid + 1;
            }
        } else if nums[mid] < target && target <= nums[n - 1] {
            left = mid + 1;
        } else {
            right = mid
        }
    }
    if nums[left] == target {
        left as i32
    } else {
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let nums = vec![4, 5, 6, 7, 0, 1, 2];
        let target = 0;
        let res = 4;
        assert_eq!(search_other(nums, target), res);
    }

    #[test]
    fn test_case_2() {
        let nums = vec![4, 5, 6, 7, 0, 1, 2];
        let target = 3;
        let res = -1;
        assert_eq!(search_other(nums, target), res);
    }

    #[test]
    fn test_case_3() {
        let nums = vec![1];
        let target = 0;
        let res = -1;
        assert_eq!(search_other(nums, target), res);
    }

    #[test]
    fn test_wrong_answer() {
        let nums = vec![4, 5, 6, 7, 0, 1, 2];
        let target = 1;
        let res = 5;
        assert_eq!(search_other(nums, target), res);
    }
}

