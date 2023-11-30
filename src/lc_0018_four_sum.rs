use std::cmp::Ordering;

pub fn four_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    let mut nums = nums;
    nums.sort();
    let mut i = 0;
    let n = nums.len();
    let mut ans = vec![];
    let target = target as i64;

    if n < 4 {
        return ans;
    }

    while i < n - 3 {
        let mut j = i + 1;
        while j < n - 2 {
            let mut k = j + 1;
            let mut l = n - 1;
            while k < l {
                let sum: i64 = nums[i] as i64 + nums[j] as i64 + nums[k] as i64 + nums[l] as i64;

                match sum.cmp(&target) {
                    Ordering::Less => k += 1,
                    Ordering::Greater => l -= 1,
                    Ordering::Equal => {
                        ans.push(vec![nums[i], nums[j], nums[k], nums[l]]);
                        k += 1;
                        l -= 1;
                        while k < l && nums[k] == nums[k - 1] {
                            k += 1;
                        }
                        while k < l && nums[l] == nums[l + 1] {
                            l -= 1;
                        }
                    }
                }
            }
            j += 1;
            while j < n - 2 && nums[j] == nums[j - 1] {
                j += 1;
            }
        }

        i += 1;
        while i < n - 3 && nums[i] == nums[i - 1] {
            i += 1;
        }
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case1() {
        let nums = vec![1, 0, -1, 0, -2, 2];
        let target = 0;
        let output = vec![vec![-2, -1, 1, 2], vec![-2, 0, 0, 2], vec![-1, 0, 0, 1]];
        assert_eq!(four_sum(nums, target), output);
    }

    #[test]
    fn test_case2() {
        let nums = vec![2, 2, 2, 2, 2];
        let target = 8;
        let output = vec![vec![2, 2, 2, 2]];
        assert_eq!(four_sum(nums, target), output);
    }

    #[test]
    fn wrong_answer() {
        let nums = vec![1000000000, 1000000000, 1000000000, 1000000000];
        let target = -294967296;
        let output: Vec<Vec<i32>> = vec![];
        assert_eq!(four_sum(nums, target), output)
    }
}
