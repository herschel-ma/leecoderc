use crate::Solution;

impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        let ss: Vec<_> = s.split_whitespace().collect();
        let p = *ss.last().unwrap();
        p.len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let s = String::from("Hello World");
        let res = 5;
        assert_eq!(Solution::length_of_last_word(s), res);
    }

    #[test]
    fn test_case_2() {
        let s = String::from("   fly me   to   the moon  ");
        let res = 4;
        assert_eq!(Solution::length_of_last_word(s), res);
    }

    #[test]
    fn test_case_3() {
        let s = String::from("luffy is still joyboy");
        let res = 6;
        assert_eq!(Solution::length_of_last_word(s), res);
    }
}

