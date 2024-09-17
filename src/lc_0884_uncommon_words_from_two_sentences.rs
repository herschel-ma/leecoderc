use crate::Solution;
use std::collections::HashMap;

impl Solution {
    pub fn uncommon_from_sentences(s1: String, s2: String) -> Vec<String> {
        s1.split_ascii_whitespace()
            .chain(s2.split_ascii_whitespace())
            .fold(HashMap::new(), |mut hm, word| {
                hm.entry(word).and_modify(|c| *c += 1).or_insert(1);
                hm
            })
            .into_iter()
            .filter_map(|(w, c)| if c == 1 { Some(w.to_string()) } else { None })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let s1 = "this apple is sweet".to_string();
        let s2 = "this apple is sour".to_string();
        let output = vec!["sweet".to_string(), "sour".to_string()];
        assert_eq!(Solution::uncommon_from_sentences(s1, s2), output);
    }

    #[test]
    fn test_case_2() {
        let s1 = "apple apple".to_string();
        let s2 = "banana".to_string();
        let output = vec!["banana".to_string()];
        assert_eq!(Solution::uncommon_from_sentences(s1, s2), output);
    }
}
