use crate::Solution;
impl Solution {
    pub fn count_consistent_strings(allowed: String, words: Vec<String>) -> i32 {
        let mut bit = [0; 26];
        allowed
            .chars()
            .for_each(|c| bit[c as usize - 'a' as usize] = 1);
        words
            .into_iter()
            .filter(|word| word.chars().all(|c| bit[c as usize - 'a' as usize] == 1))
            .count() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let allowed = "ab".to_string();
        let words = vec![
            "ad".to_string(),
            "bd".to_string(),
            "aaab".to_string(),
            "baa".to_string(),
            "badab".to_string(),
        ];
        assert_eq!(Solution::count_consistent_strings(allowed, words), 2);
    }

    #[test]
    fn test_case_2() {
        let allowed = "abc".to_string();
        let words = vec![
            "a".to_string(),
            "b".to_string(),
            "c".to_string(),
            "ab".to_string(),
            "ac".to_string(),
            "bc".to_string(),
            "abc".to_string(),
        ];
        assert_eq!(Solution::count_consistent_strings(allowed, words), 7);
    }

    #[test]
    fn test_case_3() {
        let allowed = "cad".to_string();
        let words = vec![
            "cc".to_string(),
            "acd".to_string(),
            "b".to_string(),
            "ba".to_string(),
            "bac".to_string(),
            "bad".to_string(),
            "ac".to_string(),
            "d".to_string(),
        ];
        assert_eq!(Solution::count_consistent_strings(allowed, words), 4);
    }
}
