use crate::Solution;
use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn unique_occurrences(arr: Vec<i32>) -> bool {
        let mut counter = HashMap::<i32, i32>::new();
        arr.into_iter()
            .for_each(|e| *counter.entry(e).or_insert(0) += 1);
        let len = counter.values().count();
        let counter_set = counter.values().collect::<HashSet<_>>();
        len == counter_set.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let arr = vec![1, 2, 2, 1, 1, 3];
        assert!(Solution::unique_occurrences(arr));
    }

    #[test]
    fn test_case_2() {
        let arr = vec![1, 2];
        assert!(!Solution::unique_occurrences(arr));
    }

    #[test]
    fn test_case_3() {
        let arr = vec![-3, 0, 1, -3, 1, 1, 1, -3, 10, 0];
        assert!(Solution::unique_occurrences(arr));
    }
}

