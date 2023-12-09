pub fn search_range_1(nums: Vec<i32>, target: i32) -> Vec<i32> {
    if nums.binary_search(&target).is_ok() {
        vec![
            nums.partition_point(|&i| i < target) as i32,
            nums.partition_point(|&i| i <= target) as i32 - 1,
        ]
    } else {
        vec![-1, -1]
    }
}

pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let n = nums.len();
    let search = |x| {
        let mut left = 0;
        let mut right = n;
        while left < right {
            let mid = (left + right) >> 1;
            if nums[mid] < x {
                left = mid + 1;
            } else {
                right = mid;
            }
        }
        left
    };
    let l = search(target);
    let r = search(target + 1);
    if l == r {
        return vec![-1, -1];
    }
    vec![l as i32, (r - 1) as i32]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let nums = vec![5, 7, 7, 8, 8, 10];
        let target = 8;
        let output = vec![3, 4];
        assert_eq!(search_range(nums, target), output)
    }

    #[test]
    fn test_case_2() {
        let nums = vec![5, 7, 7, 8, 8, 10];
        let target = 6;
        let output = vec![-1, -1];
        assert_eq!(search_range(nums, target), output)
    }

    #[test]
    fn test_case_3() {
        let nums = vec![];
        let target = 0;
        let output = vec![-1, -1];
        assert_eq!(search_range(nums, target), output)
    }

    #[test]
    fn test_partition_point() {
        let data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let res = data.partition_point(|&x| x < 6);
        let exp = 5;
        println!("{:?}", res);
        assert_eq!(res, exp)
    }
}
