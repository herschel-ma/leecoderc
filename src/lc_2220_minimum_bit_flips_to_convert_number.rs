use crate::Solution;

impl Solution {
    pub fn min_bit_flips(start: i32, goal: i32) -> i32 {
        (start ^ goal).count_ones() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(Solution::min_bit_flips(10, 7), 3);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(Solution::min_bit_flips(3, 4), 3);
    }
}
