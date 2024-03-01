use crate::Solution;
use std::iter;

impl Solution {
    pub fn maximum_odd_number(s: String) -> String {
        let ones = s.as_bytes().iter().filter(|&&c| c == b'1').count() - 1;
        iter::repeat('1')
            .take(ones)
            .chain(iter::repeat('0').take(s.len() - ones - 1))
            .chain(iter::once('1'))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let s = String::from("010");
        assert_eq!(Solution::maximum_odd_number(s), String::from("001"));
    }

    #[test]
    fn test_case_2() {
        let s = String::from("0101");
        assert_eq!(Solution::maximum_odd_number(s), String::from("1001"));
    }
}
