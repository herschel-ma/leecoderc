use crate::Solution;
impl Solution {
    pub fn find_the_city_1(n: i32, edges: Vec<Vec<i32>>, distance_threshold: i32) -> i32 {
        // Dijkstra 算法

        // Dijkstra 算法是一种用于计算单源最短路径的算法，适用于有向图或无向图中，边权重为非负的情况。它的基本思想是通过逐步扩展最短路径来找到从源节点到所有其他节点的最短路径。
        // 算法步骤
        //     初始化: 设置源节点的距离为 0，所有其他节点的距离为无穷大。使用一个优先队列（通常是最小堆）来选择当前最短路径的节点。
        //     选择最小距离节点: 从优先队列中取出距离最小的节点作为当前节点。
        //     更新距离: 遍历当前节点的所有邻接节点，更新其距离值，如果找到更短的路径，则更新优先队列。
        //     重复: 重复步骤 2 和 3，直到所有节点的最短路径被确定。
        //
        use std::cmp::Reverse;
        use std::collections::BinaryHeap;
        let n = n as usize;
        let adjacency_list = {
            let mut res = vec![vec![]; n];
            for edge in edges {
                let u = edge[0] as usize;
                let v = edge[1] as usize;
                let w = edge[2];
                res[u].push((v, w));
                res[v].push((u, w));
            }
            res
        };
        let mut final_results = vec![];
        for start in 0..n {
            let mut distances = vec![i32::MAX; n];
            distances[start] = 0;
            let mut primary_queue = BinaryHeap::new();
            primary_queue.push((Reverse(0), start));
            while let Some((Reverse(distance), u)) = primary_queue.pop() {
                for &(v, w) in &adjacency_list[u] {
                    let next_distance = distance + w;
                    if next_distance < distances[v] {
                        distances[v] = next_distance;
                        primary_queue.push((Reverse(next_distance), v));
                    }
                }
            }
            let count = distances
                .into_iter()
                .filter(|&x| x > 0 && x <= distance_threshold)
                .count();
            final_results.push((count, Reverse(start)));
        }
        final_results.sort();
        (final_results.first().unwrap().1).0 as i32
    }

    pub fn find_the_city_2(n: i32, edges: Vec<Vec<i32>>, distance_threshold: i32) -> i32 {
        // Floyd 算法

        // Floyd 算法（又称 Floyd-Warshall 算法）是一种用于计算所有节点对之间最短路径的算法。它可以处理图中边权为负数的情况，但不能处理有负权回路的图。
        // 算法步骤
        //     初始化: 设置一个二维数组 dist，其中 dist[i][j] 表示从节点 i 到节点 j 的距离。初始化时，如果 i 和 j 之间有边，则 dist[i][j] 为边的权重，否则为无穷大。对角线（dist[i][i]）为 0。
        //     更新: 使用三层循环，尝试通过每个节点作为中间节点来更新距离矩阵 dist[i][j]。
        //     重复: 对每个节点 k，检查是否存在更短的路径 i -> k -> j，如果存在则更新 dist[i][j]。
        let n = n as usize;
        let mut matrix = vec![vec![i32::MAX; n]; n];
        for edge in edges {
            matrix[edge[0] as usize][edge[1] as usize] = edge[2];
            matrix[edge[1] as usize][edge[0] as usize] = edge[2];
        }

        for k in 0..n {
            for i in 0..n {
                for j in 0..n {
                    if i != j && matrix[i][k] < i32::MAX && matrix[k][j] < i32::MAX {
                        matrix[i][j] = matrix[i][j].min(matrix[i][k] + matrix[k][j]);
                    }
                }
            }
        }
        let mut ans = 0;
        let mut min_count = i32::MAX;
        for i in 0..matrix.len() {
            let mut count = 0;
            for j in 0..matrix.len() {
                if matrix[i][j] <= distance_threshold {
                    count += 1;
                }
            }
            if count <= min_count {
                min_count = count;
                ans = i;
            }
        }
        ans as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    type Func = fn(n: i32, edges: Vec<Vec<i32>>, distance_threshold: i32) -> i32;
    const S1: Func = Solution::find_the_city_1;
    const S2: Func = Solution::find_the_city_2;

    #[test]
    fn test_case_1() {
        let (n, distance_threshold) = (4, 4);
        let edges = vec![vec![0, 1, 3], vec![1, 2, 1], vec![1, 3, 4], vec![2, 3, 1]];

        assert_eq!(S1(n, edges.clone(), distance_threshold), 3);
        assert_eq!(S2(n, edges, distance_threshold), 3);
    }

    #[test]
    fn test_case_2() {
        let (n, distance_threshold) = (5, 2);
        let edges = vec![
            vec![0, 1, 2],
            vec![0, 4, 8],
            vec![1, 2, 3],
            vec![1, 4, 2],
            vec![2, 3, 1],
            vec![3, 4, 1],
        ];
        assert_eq!(S1(n, edges.clone(), distance_threshold), 0);
        assert_eq!(S2(n, edges, distance_threshold), 0);
    }
}
