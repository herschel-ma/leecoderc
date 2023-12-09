use std::cmp::Ordering;

pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
    let n = nums.len();
    let mut left = 0;
    let mut right = n;
    while left < right {
        let mid = (left + right) >> 1;
        match nums[mid].cmp(&target) {
            Ordering::Less => left = mid + 1,
            Ordering::Greater => right = mid,
            Ordering::Equal => return mid as i32,
        }
    }
    left as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let nums = vec![1, 3, 5, 6];
        let target = 5;
        let output = 2;
        assert_eq!(search_insert(nums, target), output)
    }

    #[test]
    fn test_case_2() {
        let nums = vec![1, 3, 5, 6];
        let target = 2;
        let output = 1;
        assert_eq!(search_insert(nums, target), output)
    }

    #[test]
    fn test_case_3() {
        let nums = vec![1, 3, 5, 6];
        let target = 7;
        let output = 4;
        assert_eq!(search_insert(nums, target), output)
    }

    #[test]
    fn test_case_4() {
        let nums = vec![1, 3, 5, 6];
        let target = 0;
        let output = 0;
        assert_eq!(search_insert(nums, target), output)
    }
}
