use crate::Solution;

impl Solution {
    pub fn longest_common_prefix(arr1: Vec<i32>, arr2: Vec<i32>) -> i32 {
        //step1: build a hashset to store all the possible prefix in arr1
        let mut arr1_prefixes = std::collections::HashSet::new();
        for mut a1 in arr1.into_iter() {
            while a1 > 0 {
                arr1_prefixes.insert(a1);
                a1 /= 10;
            }
        }

        // step2: find the longest matching prefix in arr2
        let mut longest_prefix = 0i32;
        for mut a2 in arr2.into_iter() {
            while !arr1_prefixes.contains(&a2) && a2 > 0 {
                a2 /= 10;
            }
            if a2 > 0 {
                longest_prefix = std::cmp::max(longest_prefix, a2.ilog10() as i32 + 1)
            }
        }
        longest_prefix
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let arr1 = vec![1, 10, 100];
        let arr2 = vec![1000];
        assert_eq!(Solution::longest_common_prefix(arr1, arr2), 3);
    }

    #[test]
    fn test_case_2() {
        let arr1 = vec![1, 2, 3];
        let arr2 = vec![4, 4, 4];
        assert_eq!(Solution::longest_common_prefix(arr1, arr2), 0);
    }
}
