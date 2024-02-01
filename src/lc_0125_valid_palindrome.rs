use crate::Solution;

impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let forward: String = s
            .to_lowercase()
            .chars()
            .filter(|ch| ch.is_alphanumeric())
            .collect();

        let backward: String = forward.chars().rev().collect();

        forward == backward
    }
}

#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let s = String::from("A man, a plan, a canal: Panama");
        assert!(Solution::is_palindrome(s));
    }

    #[test]
    fn ex2() {
        let s = String::from("race a car");
        assert!(!Solution::is_palindrome(s));
    }

    #[test]
    fn ex3() {
        let s = String::from(" ");
        assert!(Solution::is_palindrome(s));
    }
}
