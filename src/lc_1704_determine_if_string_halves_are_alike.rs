use crate::Solution;

impl Solution {
    pub fn halves_are_alike(s: String) -> bool {
        fn is_vowel(c: char) -> bool {
            let vowels = "aeiouAEIOU";
            vowels.contains(c)
        }

        let s = s.split_at(s.len() / 2);

        s.0.chars()
            .filter(|&c| is_vowel(c))
            .collect::<Vec<char>>()
            .len()
            .eq(&s
                .1
                .chars()
                .filter(|&c| is_vowel(c))
                .collect::<Vec<char>>()
                .len())
    }
}

#[cfg(test)]
mod tests {
    use std::ops::Not;

    use super::*;

    #[test]
    fn test_case_1() {
        let s = String::from("book");
        assert!(Solution::halves_are_alike(s));
    }

    #[test]
    fn test_case_2() {
        let s = String::from("textbook");
        assert!(Solution::halves_are_alike(s).not());
    }

    #[test]
    fn test_case_3() {
        let s = String::from("AbCdEfGh");
        assert!(Solution::halves_are_alike(s));
    }
}

