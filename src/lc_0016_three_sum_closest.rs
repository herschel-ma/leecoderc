pub fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32 {
    let mut nums = nums;
    nums.sort();
    let n = nums.len();
    let mut ans = i32::MAX;
    for (i, v) in nums.iter().enumerate() {
        let mut j = i + 1;
        let mut k = n - 1;
        while j < k {
            let t = v + nums[j] + nums[k];
            if t == target {
                return t;
            }
            ans = if (t - target).abs() < (ans - target).abs() {
                t
            } else {
                ans
            };
            if t > target {
                k -= 1;
            } else {
                j += 1;
            }
        }
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case1() {
        let nums = vec![-1, 2, 1, -4];
        let target = 1;
        assert_eq!(three_sum_closest(nums, target), 2)
    }

    #[test]
    fn test_case2() {
        let nums = vec![0, 0, 0];
        let target = 1;
        assert_eq!(three_sum_closest(nums, target), 0)
    }
}

