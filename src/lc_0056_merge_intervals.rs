use crate::Solution;

impl Solution {
    pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut intervals = intervals;
        intervals.sort_unstable_by(|a, b| a[0].cmp(&b[0]));
        let mut res: Vec<Vec<i32>> = vec![intervals[0].clone()];
        for v in &intervals.as_slice()[1..] {
            let n = res.len();

            if v[0] > res[n - 1][1] {
                res.push(v.clone());
            } else {
                res[n - 1][1] = v[1].max(res[n - 1][1]);
            }
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    #[test]
    fn test_case_1() {
        let intervals = vec![vec![1, 3], vec![2, 6], vec![8, 10], vec![15, 18]];
        let res = vec![vec![1, 6], vec![8, 10], vec![15, 18]];
        assert_eq!(Solution::merge(intervals), res);
    }

    #[test]
    fn test_case_2() {
        let intervals = vec![vec![1, 4], vec![4, 5]];
        let res = vec![vec![1, 5]];
        assert_eq!(Solution::merge(intervals), res);
    }

    #[test]
    fn test_1_res() {
        let intervals = vec![vec![1, 4], vec![0, 4]];
        let res = vec![vec![0, 4]];
        assert_eq!(Solution::merge(intervals), res);
    }

    #[test]
    fn test_case_3() {
        let intervals = vec![
            vec![2, 3],
            vec![2, 2],
            vec![3, 3],
            vec![1, 3],
            vec![5, 7],
            vec![2, 2],
            vec![4, 6],
        ];
        let res = vec![vec![1, 3], vec![4, 7]];
        assert_eq!(Solution::merge(intervals), res);
    }
}
