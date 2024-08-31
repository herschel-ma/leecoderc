use crate::Solution;
use std::collections::BinaryHeap;

#[derive(PartialEq)]
struct Pair(f64, i32);
impl Eq for Pair {}
impl Ord for Pair {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.partial_cmp(&other.0).unwrap()
    }
}
impl PartialOrd for Pair {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Solution {
    pub fn max_probability(
        n: i32,
        edges: Vec<Vec<i32>>,
        succ_prob: Vec<f64>,
        start_node: i32,
        end_node: i32,
    ) -> f64 {
        let (mut e, mut pq, mut ans) = (
            vec![vec![]; n as usize],
            BinaryHeap::from([Pair(1f64, start_node)]),
            vec![0f64; n as usize],
        );
        for (a, b, p) in edges
            .iter()
            .zip(succ_prob.iter())
            .map(|(v, &p)| (v[0] as usize, v[1] as usize, p))
        {
            e[a].push((b, p));
            e[b].push((a, p));
        }
        while let Some(Pair(prob, cur)) = pq.pop() {
            if cur == end_node {
                break;
            }
            for &(x, p) in e[cur as usize].iter() {
                let t = prob * p;
                if t > ans[x] {
                    ans[x] = t;
                    pq.push(Pair(t, x as i32));
                }
            }
        }
        ans[end_node as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let edges = vec![vec![0, 1], vec![1, 2], vec![0, 2]];
        let succ_prob = vec![0.5, 0.5, 0.2];
        assert_eq!(
            Solution::max_probability(3, edges, succ_prob, 0, 2),
            0.25000
        );
    }

    #[test]
    fn test_case_2() {
        let edges = vec![vec![0, 1], vec![1, 2], vec![0, 2]];
        let succ_prob = vec![0.5, 0.5, 0.3];
        assert_eq!(
            Solution::max_probability(3, edges, succ_prob, 0, 2),
            0.30000
        );
    }

    #[test]
    fn test_case_3() {
        let edges = vec![vec![0, 1]];
        let succ_prob = vec![0.5];
        assert_eq!(
            Solution::max_probability(3, edges, succ_prob, 0, 2),
            0.00000
        );
    }
}
