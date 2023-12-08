pub fn next_permutation(nums: &mut Vec<i32>) {
    // step 1: 找到从右向左的第一对降序对
    let mut i = nums.len() as i32 - 2;
    while i >= 0 && nums[i as usize] >= nums[(i + 1) as usize] {
        i -= 1;
    }
    // 如果没有找到这样的一对，说明序列是降序的，真接反转整个数组
    if i == -1 {
        nums.reverse();
        return;
    }

    // step 2: 交换[i-1] 和右边第一个比它大的元素
    let mut j = nums.len() as i32 - 1;
    while nums[j as usize] <= nums[i as usize] {
        j -= 1;
    }
    nums.swap(i as usize, j as usize);

    // step 3: 反转后辍;
    nums[(i + 1) as usize..].reverse();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let mut nums: Vec<i32> = vec![1, 2, 3];
        let res: Vec<i32> = vec![1, 3, 2];
        next_permutation(&mut nums);
        assert_eq!(nums, res)
    }

    #[test]
    fn test_case_2() {
        let mut nums = vec![3, 2, 1];
        let res = vec![1, 2, 3];
        next_permutation(&mut nums);
        assert_eq!(nums, res)
    }

    #[test]
    fn test_case_3() {
        let mut nums = vec![1, 1, 5];
        let res = vec![1, 5, 1];
        next_permutation(&mut nums);
        assert_eq!(nums, res);
    }
}
