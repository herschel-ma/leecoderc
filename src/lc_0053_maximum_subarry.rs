pub fn max_sub_array_1(nums: Vec<i32>) -> i32 {
    if nums.len() == 1 {
        return nums[0];
    }
    let mut dp = Vec::with_capacity(nums.len());
    dp.push(nums[0]);
    let mut res = nums[0];
    for i in 1..nums.len() {
        if dp[i - 1] < 0 {
            dp.push(nums[i]);
        } else {
            dp.push(nums[i] + dp[i - 1]);
        }
        res = res.max(dp[i])
    }
    res
}

pub fn max_sub_array(nums: Vec<i32>) -> i32 {
    let mut ans = nums[0];
    let mut f = nums[0];
    for (_, v) in nums.iter().skip(1).enumerate() {
        f = f.max(0) + v;
        ans = ans.max(f);
    }
    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(max_sub_array(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4]), 6);
    }

    #[test]
    fn ex2() {
        assert_eq!(max_sub_array(vec![1]), 1);
    }

    #[test]
    fn ex3() {
        assert_eq!(max_sub_array(vec![5, 4, -1, 7, 8]), 23);
    }

    #[test]
    fn ex4() {
        assert_eq!(max_sub_array(vec![-2, -1]), -1)
    }
}
