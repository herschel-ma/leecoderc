use crate::Solution;
use std::collections::HashMap;

impl Solution {
    pub fn find_least_num_of_unique_ints(arr: Vec<i32>, mut k: i32) -> i32 {
        let mut ans = 0;
        let mut map: HashMap<i32, i32> = HashMap::new();
        arr.iter().for_each(|&x| {
            *map.entry(x).or_insert(0) += 1;
        });

        let mut count = map.into_iter().collect::<Vec<_>>();
        count.sort_unstable_by(|a, b| a.1.cmp(&b.1));
        count.into_iter().for_each(|(_, v)| match k - v >= 0 {
            true => k -= v,
            false => ans += 1,
        });

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(Solution::find_least_num_of_unique_ints(vec![5, 5, 4], 1), 1);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(
            Solution::find_least_num_of_unique_ints(vec![4, 3, 1, 1, 3, 3, 2], 3),
            2
        );
    }
}
