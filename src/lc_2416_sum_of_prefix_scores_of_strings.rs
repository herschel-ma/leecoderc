use crate::Solution;

#[derive(Default, Debug, Clone)]
struct TrieNode {
    children: [Option<Box<TrieNode>>; 26],
    count: i32,
}

impl TrieNode {
    fn new() -> Self {
        Default::default()
    }

    fn insert(&mut self, s: &str) {
        let mut node = self;
        for ch in s.chars() {
            if node.children[ch as usize - 'a' as usize].is_none() {
                node.children[ch as usize - 'a' as usize] = Some(Box::new(TrieNode::new()));
            }
            node.children[ch as usize - 'a' as usize]
                .as_mut()
                .unwrap()
                .count += 1;
            node = node.children[ch as usize - 'a' as usize].as_mut().unwrap();
        }
    }

    fn find(&mut self, word: &str) -> i32 {
        let mut node = self;
        let mut ans = 0;
        for ch in word.chars() {
            ans += node.children[ch as usize - 'a' as usize]
                .as_mut()
                .unwrap()
                .count;
            node = node.children[ch as usize - 'a' as usize].as_mut().unwrap();
        }
        ans
    }
}

impl Solution {
    pub fn sum_prefix_scores(words: Vec<String>) -> Vec<i32> {
        let n = words.len();
        let mut trie = TrieNode::new();
        (0..n).for_each(|i| {
            trie.insert(&words[i]);
        });

        let mut scores = vec![0; n];
        (0..n).for_each(|i| {
            scores[i] = trie.find(&words[i]);
        });
        scores
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let words = vec![
            "abc".to_string(),
            "ab".to_string(),
            "bc".to_string(),
            "b".to_string(),
        ];
        assert_eq!(Solution::sum_prefix_scores(words), vec![5, 4, 3, 2]);
    }

    #[test]
    fn test_case_2() {
        let words = vec!["abcd".to_string()];
        assert_eq!(Solution::sum_prefix_scores(words), vec![4]);
    }
}
