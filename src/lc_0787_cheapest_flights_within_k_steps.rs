use crate::Solution;
use std::collections::HashMap;

impl Solution {
    pub fn find_cheapest_price(n: i32, flights: Vec<Vec<i32>>, src: i32, dst: i32, k: i32) -> i32 {
        let mut graph = vec![vec![]; n as usize];
        for flight in flights {
            graph[flight[0] as usize].push((flight[1], flight[2]));
        }

        let mut memo = HashMap::new();

        fn recur(
            graph: &Vec<Vec<(i32, i32)>>,
            memo: &mut HashMap<(i32, i32, i32), Option<i32>>,
            src: i32,
            dst: i32,
            k: i32,
        ) -> Option<i32> {
            if let Some(res) = memo.get(&(src, dst, k)) {
                return *res;
            }
            if src == dst {
                return Some(0);
            }
            if k < 0 {
                return None;
            }
            let mut min: Option<i32> = None;
            for &(dest, price) in &graph[src as usize] {
                if let Some(res) = recur(graph, memo, dest, dst, k - 1) {
                    match min {
                        None => min = Some(res + price),
                        Some(m) => min = Some(res + price).map(|x| x.min(m)),
                    }
                }
            }

            memo.insert((src, dst, k), min);
            min
        }

        recur(&graph, &mut memo, src, dst, k).unwrap_or(-1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let n = 4;
        let flights = vec![
            vec![0, 1, 100],
            vec![1, 2, 100],
            vec![2, 0, 100],
            vec![1, 3, 600],
            vec![2, 3, 200],
        ];
        let src = 0;
        let dst = 3;
        let k = 1;
        assert_eq!(Solution::find_cheapest_price(n, flights, src, dst, k), 700);
    }

    #[test]
    fn test_case_2() {
        let n = 3;
        let flights = vec![vec![0, 1, 100], vec![1, 2, 100], vec![0, 2, 500]];
        let src = 0;
        let dst = 2;
        let k = 1;
        assert_eq!(Solution::find_cheapest_price(n, flights, src, dst, k), 200);
    }

    #[test]
    fn test_case_3() {
        let n = 3;
        let flights = vec![vec![0, 1, 100], vec![1, 2, 100], vec![0, 2, 500]];
        let src = 0;
        let dst = 2;
        let k = 0;
        assert_eq!(Solution::find_cheapest_price(n, flights, src, dst, k), 500);
    }
}
