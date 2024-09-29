use crate::Solution;

#[derive(Default, Debug, Clone)]
struct TrieNode {
    // another way to implement children: HashMap<char, TrieNode>
    children: [Option<Box<TrieNode>>; 10],
}

#[derive(Default, Debug)]
pub struct Trie {
    root: TrieNode,
}

impl Trie {
    fn new() -> Self {
        Trie {
            root: TrieNode::default(),
        }
    }

    fn insert(&mut self, num: &i32) {
        let mut node = &mut self.root;
        let num_str = num.to_string();
        for c in num_str.chars() {
            if node.children[c.to_digit(10).unwrap() as usize].is_none() {
                node.children[c.to_digit(10).unwrap() as usize] =
                    Some(Box::new(TrieNode::default()));
            }
            node = node.children[c.to_digit(10).unwrap() as usize]
                .as_mut()
                .unwrap()
        }
    }

    fn find_longest_common_prefix(&self, num: &i32) -> i32 {
        let mut node = &self.root;
        let num_str = num.to_string();
        let mut length = 0;
        for c in num_str.chars() {
            match &node.children[c.to_digit(10).unwrap() as usize] {
                Some(no) => {
                    node = no;
                    length += 1;
                }
                None => break,
            }
        }
        length
    }
}

impl Solution {
    pub fn longest_common_prefix_1(arr1: Vec<i32>, arr2: Vec<i32>) -> i32 {
        //step1: build a hashset to store all the possible prefix in arr1
        let mut arr1_prefixes = std::collections::HashSet::new();
        for mut a1 in arr1.into_iter() {
            while a1 > 0 {
                arr1_prefixes.insert(a1);
                a1 /= 10;
            }
        }

        // step2: find the longest matching prefix in arr2
        let mut longest_prefix = 0i32;
        for mut a2 in arr2.into_iter() {
            while !arr1_prefixes.contains(&a2) && a2 > 0 {
                a2 /= 10;
            }
            if a2 > 0 {
                longest_prefix = std::cmp::max(longest_prefix, a2.ilog10() as i32 + 1)
            }
        }
        longest_prefix
    }

    pub fn longest_common_prefix_2(arr1: Vec<i32>, arr2: Vec<i32>) -> i32 {
        // create a trie to store prefix
        let mut trie = Trie::new();
        for num in arr1 {
            trie.insert(&num);
        }
        let mut ans = 0;
        for num in arr2 {
            ans = ans.max(trie.find_longest_common_prefix(&num));
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    type Fn = fn(arr1: Vec<i32>, arr2: Vec<i32>) -> i32;
    const S1: Fn = Solution::longest_common_prefix_1;
    const S2: Fn = Solution::longest_common_prefix_2;

    #[test]
    fn test_case_1() {
        let arr1 = vec![1, 10, 100];
        let arr2 = vec![1000];
        assert_eq!(S1(arr1.clone(), arr2.clone()), 3);
        assert_eq!(S2(arr1, arr2), 3);
    }

    #[test]
    fn test_case_2() {
        let arr1 = vec![1, 2, 3];
        let arr2 = vec![4, 4, 4];
        assert_eq!(S1(arr1.clone(), arr2.clone()), 0);
        assert_eq!(S2(arr1, arr2), 0);
    }
}
