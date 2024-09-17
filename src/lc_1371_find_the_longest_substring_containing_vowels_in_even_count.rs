use crate::Solution;
use std::collections::HashMap;

impl Solution {
    pub fn find_the_longest_substring(s: String) -> i32 {
        let mut ans = 0;
        let mut state = 0u8;
        let mut map: HashMap<u8, i32> = HashMap::new();
        map.insert(state, -1);
        for (i, v) in s.chars().enumerate() {
            match v {
                'a' => state ^= 1 << 0,
                'e' => state ^= 1 << 1,
                'i' => state ^= 1 << 2,
                'o' => state ^= 1 << 3,
                'u' => state ^= 1 << 4,
                _ => state ^= 0,
            }
            map.entry(state).or_insert(i as i32);
            ans = ans.max(i as i32 - map[&state]);
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_case_1() {
        let s = String::from("eleetminicoworoep");
        assert_eq!(Solution::find_the_longest_substring(s), 13);
    }

    #[test]
    fn test_case_2() {
        let s = String::from("leetcodeisgreat");
        assert_eq!(Solution::find_the_longest_substring(s), 5);
    }

    #[test]
    fn test_case_3() {
        let s = String::from("bcbcbc");
        assert_eq!(Solution::find_the_longest_substring(s), 6);
    }
}
