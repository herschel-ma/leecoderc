use crate::Solution;

impl Solution {
    pub fn halves_are_alike(s: String) -> bool {
        let (s1, s2) = s.split_at(s.len() / 2);

        s1.chars()
            .filter(|&c| "aeiouAEIOU".contains(c))
            .count()
            .eq(&s2.chars().filter(|&c| "aeiouAEIOU".contains(c)).count())
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

