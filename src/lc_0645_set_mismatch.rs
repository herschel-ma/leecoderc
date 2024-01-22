use crate::Solution;

impl Solution {
    pub fn find_error_nums(nums: Vec<i32>) -> Vec<i32> {
        let mut res = vec![0; 2];
        let mut counter = vec![0; 10_000];
        nums.iter()
            .for_each(|&num| counter[(num - 1) as usize] += 1);
        counter[..nums.len()]
            .iter()
            .zip(1..)
            .for_each(|(count, num)| match &count {
                2 => res[0] = num,
                0 => res[1] = num,
                _ => (),
            });
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let nums = vec![1, 2, 2, 4];
        assert_eq!(Solution::find_error_nums(nums), vec![2, 3]);
    }

    #[test]
    fn test_case_2() {
        let nums = vec![1, 1];
        assert_eq!(Solution::find_error_nums(nums), vec![1, 2]);
    }
}

