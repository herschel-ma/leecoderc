use crate::Solution;

impl Solution {
    pub fn generate_matrix(n: i32) -> Vec<Vec<i32>> {
        let mut res = vec![vec![0; n as usize]; n as usize];
        let (mut i, mut j, mut k) = (0_i32, 0_i32, 0);
        let dirs: &[(i32, i32)] = &[(0, 1), (1, 0), (0, -1), (-1, 0)];
        for v in 1..=n * n {
            res[i as usize][j as usize] = v;
            let (mut x, mut y) = (i + dirs[k].0, j + dirs[k].1);
            if x < 0 || y < 0 || x >= n || y >= n || res[x as usize][y as usize] != 0 {
                k = (k + 1) % 4;
                x = i + dirs[k].0;
                y = j + dirs[k].1;
            }

            i = x;
            j = y;
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let n = 3;
        let output = vec![vec![1, 2, 3], vec![8, 9, 4], vec![7, 6, 5]];
        assert_eq!(Solution::generate_matrix(n), output);
    }

    #[test]
    fn test_case_2() {
        let n = 1;
        let output = vec![vec![1]];
        assert_eq!(Solution::generate_matrix(n), output);
    }
}

