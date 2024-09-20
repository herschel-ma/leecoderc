use crate::Solution;
impl Solution {
    pub fn shortest_palindrome(s: String) -> String {
        let rev_str = s.chars().rev().collect::<String>();
        for i in 0..s.len() {
            if s.starts_with(&rev_str[i..]) {
                return format!("{}{}", &rev_str[..i], s);
            }
        }
        "".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let s = "aacecaaa".to_string();
        let output = "aaacecaaa".to_string();
        assert_eq!(Solution::shortest_palindrome(s), output);
    }

    #[test]
    fn test_case_2() {
        let s = "abcd".to_string();
        let output = "dcbabcd".to_string();
        assert_eq!(Solution::shortest_palindrome(s), output);
    }
}
