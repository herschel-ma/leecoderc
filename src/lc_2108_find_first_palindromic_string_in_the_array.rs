use crate::Solution;

impl Solution {
    pub fn first_palindrome(words: Vec<String>) -> String {
        fn is_palindrome(s: &str) -> bool {
            let len = s.len();
            let s = s.chars().collect::<Vec<char>>();
            if len <= 1 {
                return true;
            }
            let mut i = 0;
            let mut j = len - 1;
            while i < j {
                if s[i] != s[j] {
                    return false;
                }
                i += 1;
                j -= 1;
            }
            true
        }
        words
            .into_iter()
            .find(|s| is_palindrome(&s))
            .unwrap_or(String::from(""))
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tets_case_1() {
        let words = vec![
            String::from("abc"),
            String::from("car"),
            String::from("ada"),
            String::from("racecar"),
            String::from("cool"),
        ];
        let output = String::from("ada");
        assert_eq!(Solution::first_palindrome(words), output);
    }

    #[test]
    fn test_case_3() {
        let words = vec![String::from("notapalindrome"), String::from("racecar")];
        let output = String::from("racecar");
        assert_eq!(Solution::first_palindrome(words), output);
    }

    #[test]
    fn test_case_2() {
        let words = vec![String::from("def"), String::from("ghi")];
        let output = String::from("");
        assert_eq!(Solution::first_palindrome(words), output);
    }
}
