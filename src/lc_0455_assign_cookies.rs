use crate::Solution;
use std::collections::BinaryHeap;

impl Solution {
    pub fn find_content_children_loop(g: Vec<i32>, s: Vec<i32>) -> i32 {
        let (m, n) = (g.len(), s.len());
        let (mut g, mut s) = (g, s);
        let (mut i, mut j) = (0, 0);
        g.sort();
        s.sort();

        let mut ans = 0;
        while i < m && j < n {
            if s[j] >= g[i] {
                i += 1;
                ans += 1;
            }
            j += 1;
        }
        ans
    }

    pub fn find_content_children_heap(g: Vec<i32>, s: Vec<i32>) -> i32 {
        let mut children = BinaryHeap::from(g);
        let mut cookies = BinaryHeap::from(s);
        let total = cookies.len();

        while let (Some(greedy), Some(size)) = (children.pop(), cookies.peek()) {
            if greedy <= *size {
                cookies.pop();
            }
        }

        (total - cookies.len()) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    type Func = fn(Vec<i32>, Vec<i32>) -> i32;

    const FUNC: Func = Solution::find_content_children_heap as Func;
    #[test]
    fn test_case_1() {
        let g = vec![1, 2, 3];
        let s = vec![1, 1];
        assert_eq!(FUNC(g, s), 1);
    }

    #[test]
    fn test_case_2() {
        let g = vec![1, 2];
        let s = vec![1, 2, 3];
        assert_eq!(FUNC(g, s), 2);
    }
}

