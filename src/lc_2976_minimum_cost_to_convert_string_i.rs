use crate::Solution;
use std::{cmp::Reverse, collections::BinaryHeap};

impl Solution {
    pub fn minimum_cost_1(
        source: String,
        target: String,
        original: Vec<char>,
        changed: Vec<char>,
        cost: Vec<i32>,
    ) -> i64 {
        // Dijkstra 算法

        pub fn search_minimum_cost(i: usize, j: usize, matrix: &[Vec<i32>]) -> i32 {
            let mut hp = BinaryHeap::new();
            let mut d = [i32::MAX; 26];
            let mut v = [false; 26];
            d[i] = 0;
            hp.push((Reverse(0), i));

            while let Some(t) = hp.pop() {
                if v[t.1] {
                    continue;
                }

                if t.1 == j {
                    return d[j];
                }
                v[t.1] = true;

                for k in 0..26 {
                    if matrix[t.1][k] < i32::MAX && d[t.1] + matrix[t.1][k] < d[k] {
                        d[k] = d[t.1] + matrix[t.1][k];
                        hp.push((Reverse(d[k]), k));
                    }
                }
            }
            -1
        }

        let mut matrix = vec![vec![i32::MAX; 26]; 26];
        for i in 0..original.len() {
            let j = (original[i] as u8 - b'a') as usize;
            let k = (changed[i] as u8 - b'a') as usize;
            matrix[j][k] = matrix[j][k].min(cost[i]);
        }
        let sb = source.as_bytes();
        let tb = target.as_bytes();
        let mut r = 0;

        for k in 0..sb.len() {
            let i = (sb[k] - b'a') as usize;
            let j = (tb[k] - b'a') as usize;
            let c = search_minimum_cost(i, j, &matrix);
            if c == -1 {
                return -1;
            }
            r += c as i64;
        }
        r
    }

    pub fn minimum_cost_2(
        source: String,
        target: String,
        original: Vec<char>,
        changed: Vec<char>,
        cost: Vec<i32>,
    ) -> i64 {
        // Floyd 算法
        let mut matrix = vec![vec![i32::MAX as u64; 26]; 26];
        for i in 0..original.len() {
            let o = (original[i] as u8 - b'a') as usize;
            let t = (changed[i] as u8 - b'a') as usize;
            matrix[o][t] = matrix[o][t].min(cost[i] as u64);
        }

        for k in 0..26 {
            for i in 0..26 {
                for j in 0..26 {
                    matrix[i][j] = matrix[i][j].min(matrix[i][k] + matrix[k][j]);
                }
            }
        }

        let mut res = 0;
        let source = source.into_bytes();
        let target = target.into_bytes();
        for i in 0..source.len() {
            if target[i] != source[i] {
                let v = matrix[(source[i] - b'a') as usize][(target[i] - b'a') as usize];
                if v >= i32::MAX as u64 {
                    return -1;
                }
                res += v
            }
        }
        res as i64
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    type Func = fn(
        source: String,
        target: String,
        original: Vec<char>,
        changed: Vec<char>,
        cost: Vec<i32>,
    ) -> i64;
    const S1: Func = Solution::minimum_cost_1;
    const S2: Func = Solution::minimum_cost_2;

    mod s1 {
        use super::*;

        #[test]
        fn test_case_1_s1() {
            let source = String::from("abcd");
            let target = String::from("acbe");
            let original = vec!['a', 'b', 'c', 'c', 'e', 'd'];
            let changed = vec!['b', 'c', 'b', 'e', 'b', 'e'];
            let cost = vec![2, 5, 5, 1, 2, 20];
            assert_eq!(S1(source, target, original, changed, cost), 28);
        }
        #[test]
        fn test_case_2_s1() {
            let source = String::from("aaaa");
            let target = String::from("bbbb");
            let original = vec!['a', 'c'];
            let changed = vec!['c', 'b'];
            let cost = vec![1, 2];
            assert_eq!(S1(source, target, original, changed, cost), 12);
        }
        #[test]
        fn test_case_3_s1() {
            let source = String::from("abcd");
            let target = String::from("abce");
            let original = vec!['a'];
            let changed = vec!['e'];
            let cost = vec![10000];
            assert_eq!(S1(source, target, original, changed, cost), -1);
        }
    }

    mod s2 {
        use super::*;
        #[test]
        fn test_case_1_s2() {
            let source = String::from("abcd");
            let target = String::from("acbe");
            let original = vec!['a', 'b', 'c', 'c', 'e', 'd'];
            let changed = vec!['b', 'c', 'b', 'e', 'b', 'e'];
            let cost = vec![2, 5, 5, 1, 2, 20];
            assert_eq!(S2(source, target, original, changed, cost), 28);
        }
        #[test]
        fn test_case_2_s2() {
            let source = String::from("aaaa");
            let target = String::from("bbbb");
            let original = vec!['a', 'c'];
            let changed = vec!['c', 'b'];
            let cost = vec![1, 2];
            assert_eq!(S2(source, target, original, changed, cost), 12);
        }
        #[test]
        fn test_case_3_s2() {
            let source = String::from("abcd");
            let target = String::from("abce");
            let original = vec!['a'];
            let changed = vec!['e'];
            let cost = vec![10000];
            assert_eq!(S2(source, target, original, changed, cost), -1);
        }
    }
}
