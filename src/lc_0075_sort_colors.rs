pub struct Solution {}

impl Solution {
    pub fn sort_colors(nums: &mut Vec<i32>) {
        let n = nums.len();
        if n <= 1 {
            return;
        }
        let (mut left, mut right, mut cur) = (0, n - 1, 0);
        while cur <= right {
            match nums[cur] {
                0 => {
                    nums.swap(cur, left);
                    left += 1;
                    cur += 1;
                }
                2 => {
                    nums.swap(cur, right);
                    if right == 0 {
                        break;
                    }
                    right -= 1;
                }
                _ => cur += 1,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_colors() {
        let mut nums = vec![2, 0, 2, 1, 1, 0];
        Solution::sort_colors(&mut nums);
        assert_eq!(vec![0, 0, 1, 1, 2, 2], nums)
    }
    #[test]
    fn test_sort_colors2() {
        let mut nums = vec![2, 0, 1];
        Solution::sort_colors(&mut nums);
        assert_eq!(vec![0, 1, 2], nums)
    }
    #[test]
    fn test_sort_colors3() {
        let mut nums = vec![2];
        Solution::sort_colors(&mut nums);
        assert_eq!(vec![2], nums)
    }
    #[test]
    fn test_sort_colors4() {
        let mut nums = vec![2, 2];
        Solution::sort_colors(&mut nums);
        assert_eq!(vec![2, 2], nums)
    }
}
