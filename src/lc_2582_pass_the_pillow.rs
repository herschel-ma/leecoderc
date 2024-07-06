use crate::Solution;
use std::cmp::Ordering;

impl Solution {
    pub fn pass_the_pillow(n: i32, time: i32) -> i32 {
        let mut time = time;
        let mut t = 1;
        let mut k = 1;
        while time > 0 {
            t += k;
            if t.cmp(&n) == Ordering::Equal || t.cmp(&1) == Ordering::Equal {
                k *= -1;
            };
            time -= 1;
        }
        t
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pass_the_pillow_1() {
        let n = 3;
        let time = 2;
        assert_eq!(Solution::pass_the_pillow(n, time), 3);
    }

    #[test]
    fn test_pass_the_pillow_2() {
        let n = 4;
        let time = 5;
        assert_eq!(Solution::pass_the_pillow(n, time), 2);
    }
}
