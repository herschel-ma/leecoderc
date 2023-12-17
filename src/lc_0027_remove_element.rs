pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    let mut k = 0;
    for i in 0..nums.len() {
        if nums[i] != val {
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
        let mut nums = vec![3, 2, 2, 3];
        let val = 3;
        let output = 2;
        let res = [2, 2];
        assert_eq!(remove_element(&mut nums, val), output);
        for ele in res.iter() {
            assert!(nums[..output as usize].contains(ele))
        }
    }

    #[test]
    fn test_case2() {
        let mut nums = vec![0, 1, 2, 2, 3, 0, 4, 2];
        let val = 2;
        let output = 5;
        let res = [0, 1, 4, 0, 3];
        assert_eq!(remove_element(&mut nums, val), output);
        for ele in res.iter() {
            assert!(nums[..output as usize].contains(ele));
        }
    }
}
