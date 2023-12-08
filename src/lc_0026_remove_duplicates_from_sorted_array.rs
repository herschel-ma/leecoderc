pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    let mut k = 0;
    for i in 0..nums.len() {
        if k == 0 || nums[i] != nums[k - 1] {
            nums[k] = nums[i];
            k += 1;
        }
    }
    k as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case1() {
        let mut nums = vec![1, 1, 2];
        let output = 2;
        let res = vec![1, 2];
        assert_eq!(remove_duplicates(&mut nums), output);
        assert_eq!(nums[..output as usize], res)
    }

    #[test]
    fn test_case2() {
        let mut nums = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
        let output = 5;
        let res = vec![0, 1, 2, 3, 4];
        assert_eq!(remove_duplicates(&mut nums), output);
        assert_eq!(nums[..output as usize], res)
    }
}
