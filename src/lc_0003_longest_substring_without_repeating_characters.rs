pub struct Solution;

impl Solution {
    fn length_of_longest_substring(s: String) -> i32 {
        use std::collections::HashSet;
        let array = s.chars().collect::<Vec<char>>();

        let mut hs = HashSet::new();

        let mut start = 0;
        let mut end = 0;
        let mut max_len = 0;
        while end < array.len() {
            if !hs.insert(array[end]) {
                while array[start] != array[end] {
                    hs.remove(&array[start]);
                    start += 1;
                }
                start += 1;
            }
            end += 1;
            max_len = max_len.max(hs.len());
        }

        max_len as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(
            Solution::length_of_longest_substring("abcabcbb".to_string()),
            3
        );
    }

    #[test]
    fn ex2() {
        assert_eq!(
            Solution::length_of_longest_substring("bbbbb".to_string()),
            1
        );
    }

    #[test]
    fn ex3() {
        assert_eq!(
            Solution::length_of_longest_substring("pwwkew".to_string()),
            3
        );
    }
}
