use std::{cmp::Reverse, collections::BinaryHeap};

use crate::Solution;

impl Solution {
    pub fn modified_graph_edges(
        n: i32,
        mut edges: Vec<Vec<i32>>,
        source: i32,
        destination: i32,
        target: i32,
    ) -> Vec<Vec<i32>> {
        fn dijkstra_algorithm(n: i32, edges: &[Vec<i32>], src: i32, dst: i32) -> i32 {
            let mut g = vec![vec![]; n as usize];
            let mut pq = BinaryHeap::from([(Reverse(0), src)]);
            let mut cost_arr = vec![i32::MAX; n as usize];
            for e in edges.iter() {
                if e[2] == -1 {
                    continue;
                }
                let (a, b, w) = (e[0], e[1], e[2]);
                g[a as usize].push((b as usize, w));
                g[b as usize].push((a as usize, w));
            }
            while let Some(x) = pq.pop() {
                let (Reverse(prev_cost), prev_node) = x;
                if prev_node == dst {
                    return prev_cost;
                }
                for &(next_node, cost) in &g[prev_node as usize] {
                    if cost + prev_cost < cost_arr[next_node] {
                        cost_arr[next_node] = prev_cost + cost;
                        pq.push((Reverse(cost_arr[next_node]), next_node as i32));
                    }
                }
            }
            i32::MAX
        }

        // Transfer data's type
        let (n, src, dst) = (n as usize, source as usize, destination as usize);

        // Construct a topo graph
        let mut adj = vec![vec![]; n];
        for idx in 0..edges.len() {
            if edges[idx][2] == -1 {
                continue;
            }

            adj[edges[idx][0] as usize].push((edges[idx][1] as usize, edges[idx][2]));
            adj[edges[idx][1] as usize].push((edges[idx][0] as usize, edges[idx][2]));
        }

        // Find the shortest path with Dijkstra's algorithm
        let path_cost = dijkstra_algorithm(n as i32, &edges, src as i32, dst as i32);
        if path_cost < target {
            return Vec::new();
        }

        // We find the path cost match the target, then we set another edges's cost to
        // maximum number
        if path_cost == target {
            (0..edges.len()).for_each(|idx| {
                if edges[idx][2] == -1 {
                    edges[idx][2] = 2000000000;
                }
            });

            return edges;
        }

        // Change every edge's cost and recalculated it
        for idx in 0..edges.len() {
            if edges[idx][2] != -1 {
                continue;
            }

            // Set the current edge's cost to 1
            edges[idx][2] = 1;

            // Push the new edge we can reach, and recaculate the path
            adj[edges[idx][0] as usize].push((edges[idx][1] as usize, 1));
            adj[edges[idx][1] as usize].push((edges[idx][0] as usize, 1));

            let new_cost = dijkstra_algorithm(n as i32, &edges, src as i32, dst as i32);

            if new_cost <= target {
                edges[idx][2] += target - new_cost;

                // Set all the edges with negative cost to 1
                (idx + 1..edges.len()).for_each(|jdx| {
                    if edges[jdx][2] == -1 {
                        edges[jdx][2] = 2000000000;
                    }
                });

                return edges;
            }
        }

        Vec::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let edges = vec![
            vec![4, 1, -1],
            vec![2, 0, -1],
            vec![0, 3, -1],
            vec![4, 3, -1],
        ];
        let v = Solution::modified_graph_edges(5, edges, 0, 1, 5);
        assert!(
            v == vec![vec![4, 1, 1], vec![2, 0, 1], vec![0, 3, 3], vec![4, 3, 1]]
                || v == vec![vec![4, 1, 1], vec![2, 0, 1], vec![0, 3, 1], vec![4, 3, 3]]
                || v == vec![vec![4, 1, 3], vec![2, 0, 1], vec![0, 3, 1], vec![4, 3, 1]]
        );
    }
    #[test]
    fn test_case_2() {
        let edges = vec![vec![0, 1, -1], vec![0, 2, 5]];
        let v: Vec<Vec<i32>> = Vec::new();
        assert_eq!(Solution::modified_graph_edges(3, edges, 0, 2, 6), v);
    }

    #[test]
    fn test_case_3() {
        let edges = vec![vec![1, 0, 4], vec![1, 2, 3], vec![2, 3, 5], vec![0, 3, -1]];
        assert_eq!(
            Solution::modified_graph_edges(4, edges, 0, 2, 6),
            vec![vec![1, 0, 4], vec![1, 2, 3], vec![2, 3, 5], vec![0, 3, 1]]
        )
    }
}
