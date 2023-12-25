use crate::Solution;
use std::collections::HashMap;

impl Solution {
    pub fn is_scramble(s1: String, s2: String) -> bool {
        let mut map = HashMap::new();

        fn f<'a>(s1: &'a str, s2: &'a str, m: &mut HashMap<(&'a str, &'a str), bool>) -> bool {
            if m.contains_key(&(s1, s2)) {
                return *m.get(&(s1, s2)).unwrap();
            }

            if s1 == s2 {
                m.insert((s1, s2), true);
                return true;
            }

            let n = s1.len();
            if n == 1 {
                return false;
            }

            for i in 1..n {
                if f(&s1[..i], &s2[..i], m) && f(&s1[i..], &s2[i..], m)
                    || f(&s1[..i], &s2[n - i..], m) && f(&s1[i..], &s2[..n - i], m)
                {
                    m.insert((s1, s2), true);
                    return true;
                } else {
                    m.insert((s1, s2), false);
                }
            }

            false
        }

        f(&s1[..], &s2[..], &mut map)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let s1 = String::from("great");
        let s2 = String::from("rgeat");
        assert!(Solution::is_scramble(s1, s2));
    }

    #[test]
    fn test_case_2() {
        let s1 = String::from("abcde");
        let s2 = String::from("caebd");
        assert!(!Solution::is_scramble(s1, s2));
    }

    #[test]
    fn test_case_3() {
        let s1 = String::from("a");
        let s2 = String::from("a");
        assert!(Solution::is_scramble(s1, s2));
    }
}
