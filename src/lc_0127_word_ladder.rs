use crate::Solution;
use std::collections::HashSet;
use std::collections::VecDeque;

impl Solution {
    pub fn ladder_length(begin_word: String, end_word: String, word_list: Vec<String>) -> i32 {
        let begin_word: Vec<char> = begin_word.chars().collect();
        let end_word: Vec<char> = end_word.chars().collect();

        let word_list: Vec<Vec<char>> = word_list
            .into_iter()
            .map(|s| s.chars().collect::<Vec<char>>())
            .collect();
        let mut word_set = word_list.into_iter().collect::<HashSet<Vec<char>>>();
        if !word_set.contains(&end_word) {
            return 0;
        }

        // use bfs to get the shorted path.
        let mut q: VecDeque<(Vec<char>, usize)> = VecDeque::new();
        q.push_back((begin_word, 1));

        while !q.is_empty() {
            let (word, depth) = q.pop_front().unwrap();
            if word == end_word {
                return depth as i32;
            }

            let neibs = Self::collect(word, &mut word_set);
            for neib in neibs {
                q.push_back((neib, depth + 1));
            }
        }

        0
    }

    pub fn collect(s: Vec<char>, dict: &mut HashSet<Vec<char>>) -> Vec<Vec<char>> {
        let mut ret: Vec<Vec<char>> = Vec::new();

        for i in 0..s.len() {
            let mut cp = s.clone();
            for j in 0..26 {
                if cp[i] == (b'a' + j) as char {
                    continue;
                }
                cp[i] = (b'a' + j) as char;
                if dict.contains(&cp) {
                    dict.remove(&cp);
                    ret.push(cp.clone());
                }
            }
        }

        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let begin_word = "hit".to_string();
        let end_word = "cog".to_string();
        let word_list = vec![
            "hot".to_string(),
            "dot".to_string(),
            "dog".to_string(),
            "lot".to_string(),
            "log".to_string(),
            "cog".to_string(),
        ];
        assert_eq!(Solution::ladder_length(begin_word, end_word, word_list), 5);
    }

    #[test]
    fn test_case_2() {
        let begin_word = "hit".to_string();
        let end_word = "cog".to_string();
        let word_list = vec![
            "hot".to_string(),
            "dot".to_string(),
            "dog".to_string(),
            "lot".to_string(),
            "log".to_string(),
        ];
        assert_eq!(Solution::ladder_length(begin_word, end_word, word_list), 0);
    }
}

