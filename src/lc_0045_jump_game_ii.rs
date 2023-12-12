pub fn jump(nums: Vec<i32>) -> i32 {
    let (mut last, mut mx, mut ans) = (0, 0, 0);
    for (i, v) in nums[..nums.len() - 1].iter().enumerate() {
        mx = mx.max(i as i32 + v);
        if last == i {
            ans += 1;
            last = mx as usize;
        }
    }
    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let nums = vec![2, 3, 1, 1, 4];
        let output = 2;
        assert_eq!(jump(nums), output);
    }
    #[test]
    fn test_case_2() {
        let nusm = vec![2, 3, 0, 1, 4];
        let output = 2;
        assert_eq!(jump(nusm), output);
    }
}
