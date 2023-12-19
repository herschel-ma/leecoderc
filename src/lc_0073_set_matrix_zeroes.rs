use crate::Solution;

impl Solution {
    pub fn set_zeroes(matrix: &mut [Vec<i32>]) {
        let mut zero_rows = Vec::new();
        let mut zero_cols = Vec::new();

        for (i, row) in matrix.iter().enumerate() {
            for (j, v) in row.iter().enumerate() {
                if *v == 0 {
                    zero_rows.push(i);
                    zero_cols.push(j);
                }
            }
        }

        for i in zero_rows {
            matrix[i].iter_mut().for_each(|x| *x = 0);
        }

        for j in zero_cols {
            for r in matrix.iter_mut() {
                r[j] = 0;
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_case_1() {
        let mut matrix = vec![vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]];
        let output = vec![vec![1, 0, 1], vec![0, 0, 0], vec![1, 0, 1]];
        Solution::set_zeroes(&mut matrix);
        assert_eq!(matrix, output);
    }

    #[test]
    fn test_case_2() {
        let mut matrix = vec![vec![0, 1, 2, 0], vec![3, 4, 5, 2], vec![1, 3, 1, 5]];
        let output = vec![vec![0, 0, 0, 0], vec![0, 4, 5, 0], vec![0, 3, 1, 0]];
        Solution::set_zeroes(&mut matrix);
        assert_eq!(matrix, output);
    }
}
