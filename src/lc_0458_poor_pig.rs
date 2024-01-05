use crate::Solution;

impl Solution {
    pub fn poor_pigs(buckets: i32, minutes_to_die: i32, minutes_to_test: i32) -> i32 {
        let base = minutes_to_test / minutes_to_die + 1;
        (buckets as f32).log(base as f32).ceil() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(Solution::poor_pigs(1000, 15, 60), 5);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(Solution::poor_pigs(4, 15, 15), 2);
    }
    #[test]
    fn test_case_3() {
        assert_eq!(Solution::poor_pigs(4, 15, 30), 2);
    }
    #[test]
    fn test_case_4() {
        assert_eq!(Solution::poor_pigs(125, 1, 4), 3);
    }
}

