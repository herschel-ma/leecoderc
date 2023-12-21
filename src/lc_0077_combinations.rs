use crate::Solution;

impl Solution {
    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        let mut res: Vec<Vec<i32>> = Vec::new();
        let mut cur: Vec<i32> = Vec::new();

        Self::dfs(1, &mut cur, &mut res, n, k);
        res
    }

    fn dfs(i: i32, cur: &mut Vec<i32>, res: &mut Vec<Vec<i32>>, n: i32, k: i32) {
        if cur.len() == k as usize {
            res.push(cur.clone());
        }
        if i > n {
            return;
        }
        for j in i..=n {
            cur.push(j);
            Self::dfs(j + 1, cur, res, n, k);
            cur.pop();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let n = 4;
        let k = 2;
        let res = vec![
            vec![1, 2],
            vec![1, 3],
            vec![1, 4],
            vec![2, 3],
            vec![2, 4],
            vec![3, 4],
        ];
        assert_eq!(Solution::combine(n, k), res);
    }

    #[test]
    fn test_case_2() {
        let n = 1;
        let k = 1;
        let res = vec![vec![1]];
        assert_eq!(Solution::combine(n, k), res);
    }
}

