use std::collections::HashMap;

use crate::Solution;

impl Solution {
    pub fn max_length_between_equal_characters(s: String) -> i32 {
        let mut res = -1;
        let mut map: HashMap<char, usize> = HashMap::new();
        for (i, c) in s.chars().enumerate() {
            if let Some(index) = map.get(&c) {
                res = res.max((i - index - 1) as i32);
            } else {
                map.insert(c, i);
            }
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let s = "aa".into();
        assert_eq!(Solution::max_length_between_equal_characters(s), 0);
    }

    #[test]
    fn test_case_2() {
        let s = "abca".into();
        assert_eq!(Solution::max_length_between_equal_characters(s), 2);
    }

    #[test]
    fn test_case_3() {
        let s = "cbzxy".into();
        assert_eq!(Solution::max_length_between_equal_characters(s), -1);
    }
}

