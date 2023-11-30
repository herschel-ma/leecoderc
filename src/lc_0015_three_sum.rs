use std::cmp::Ordering;
use std::collections::HashSet;

#[allow(dead_code)]
pub fn three_sum_1(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut nums = nums;
    nums.sort();
    let n = nums.len();
    let mut res: HashSet<Vec<i32>> = HashSet::new();
    for i in 0..n - 2 {
        let n1 = nums[i];
        let mut j = i + 1;
        let mut k = n - 1;
        while j < k && n1 <= 0 {
            let n2 = nums[j];
            let n3 = nums[k];
            let sum = n1 + n2 + n3;
            match sum.cmp(&0) {
                Ordering::Less => j += 1,
                Ordering::Greater => k -= 1,
                Ordering::Equal => {
                    res.insert(vec![n1, n2, n3]);
                    while j < k && nums[j] == n2 {
                        j += 1;
                    }
                    while j < k && nums[k] == n3 {
                        k -= 1;
                    }
                }
            }
        }
    }
    res.into_iter().collect::<Vec<Vec<i32>>>()
}

pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut nums = nums;
    nums.sort();
    let n = nums.len();
    let mut res = vec![];
    let mut i = 0;
    while i < n - 2 && nums[i] <= 0 {
        let mut l = i + 1;
        let mut r = n - 1;
        while l < r {
            match (nums[i] + nums[l] + nums[r]).cmp(&0) {
                Ordering::Less => {
                    l += 1;
                }
                Ordering::Greater => {
                    r -= 1;
                }
                Ordering::Equal => {
                    res.push(vec![nums[i], nums[l], nums[r]]);
                    l += 1;
                    r -= 1;
                    while l < n && nums[l] == nums[l - 1] {
                        l += 1;
                    }
                    while r > 0 && nums[r] == nums[r + 1] {
                        r -= 1;
                    }
                }
            }
        }
        i += 1;
        while i < n - 2 && nums[i] == nums[i - 1] {
            i += 1;
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let nums = vec![-1, 0, 1, 2, -1, -4];
        let output = vec![vec![-1, -1, 2], vec![-1, 0, 1]];
        assert_eq!(three_sum(nums), output)
    }

    #[test]
    fn case2() {
        let nums = vec![0, 1, 1];
        let output: Vec<Vec<i32>> = Vec::new();
        assert_eq!(three_sum(nums), output)
    }
}
