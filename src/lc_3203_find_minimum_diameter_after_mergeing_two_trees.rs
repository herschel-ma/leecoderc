use crate::Solution;
use std::collections::HashMap;

impl Solution {
    pub fn minimum_diameter_after_merge(edges1: Vec<Vec<i32>>, edges2: Vec<Vec<i32>>) -> i32 {
        let d1 = Self::tree_diameter(edges1);
        let d2 = Self::tree_diameter(edges2);
        d1.max(d2).max((d1 + 1) / 2 + (d2 + 1) / 2 + 1)
    }

    // return the diameter of tree
    pub fn tree_diameter(edges: Vec<Vec<i32>>) -> i32 {
        let mut g: HashMap<i32, Vec<i32>> = HashMap::new();
        // build adjacency list
        for edge in edges {
            let a = edge[0];
            let b = edge[1];
            g.entry(a).or_default().push(b);
            g.entry(b).or_default().push(a);
        }

        struct DfsState {
            ans: i32,
            a: i32,
        }

        fn dfs(i: i32, fa: i32, t: i32, g: &HashMap<i32, Vec<i32>>, state: &mut DfsState) {
            if let Some(neighbors) = g.get(&i) {
                for &j in neighbors {
                    if j != fa {
                        dfs(j, i, t + 1, g, state);
                    }
                }
            }
            if state.ans < t {
                state.ans = t;
                state.a = i;
            }
        }
        let mut state = DfsState { ans: 0, a: 0 };
        // First DFS to find the farthest point
        dfs(0, -1, 0, &g, &mut state);
        // Second DFS to find the farthest point to find the diameter
        let start_point = state.a;
        state.ans = 0;
        dfs(start_point, -1, 0, &g, &mut state);
        state.ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let edges1 = vec![vec![0, 1], vec![0, 2], vec![0, 3]];
        let edges2 = vec![vec![0, 1]];
        assert_eq!(Solution::minimum_diameter_after_merge(edges1, edges2), 3);
    }

    #[test]
    fn test_case_2() {
        let edges1 = vec![
            vec![0, 1],
            vec![0, 2],
            vec![0, 3],
            vec![2, 4],
            vec![2, 5],
            vec![3, 6],
            vec![2, 7],
        ];
        let edges2 = vec![
            vec![0, 1],
            vec![0, 2],
            vec![0, 3],
            vec![2, 4],
            vec![2, 5],
            vec![3, 6],
            vec![2, 7],
        ];
        assert_eq!(Solution::minimum_diameter_after_merge(edges1, edges2), 5);
    }
}
