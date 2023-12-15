pub struct Solution {}

impl Solution {
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let row = matrix.len();
        if row == 0 || matrix[0].is_empty() {
            return vec![];
        }
        let col = matrix[0].len();
        let mut res = matrix[0].clone();
        if row > 1 {
            for i in matrix.iter().take(row).skip(1) {
                res.push(i[col - 1]);
            }
            for j in (0..col - 1).rev() {
                res.push(matrix[row - 1][j]);
            }
            if col > 1 {
                for i in matrix.iter().take(row - 1).skip(1).rev() {
                    res.push(i[0])
                }
            }
        }

        let mut m: Vec<Vec<i32>> = vec![];
        for k in matrix.iter().take(row - 1).skip(1) {
            let t = if col > 1 {
                k[1..col - 1].to_owned()
            } else {
                vec![]
            };
            m.push(t);
        }
        res.extend(Self::spiral_order(m));
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let matrix = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let res = vec![1, 2, 3, 6, 9, 8, 7, 4, 5];
        assert_eq!(Solution::spiral_order(matrix), res);
    }

    #[test]
    fn test_case_2() {
        let matrix = vec![vec![1, 2, 3, 4], vec![5, 6, 7, 8], vec![9, 10, 11, 12]];
        let res = vec![1, 2, 3, 4, 8, 12, 11, 10, 9, 5, 6, 7];
        assert_eq!(Solution::spiral_order(matrix), res);
    }
    #[test]
    fn test_case_3() {
        let matrix = vec![vec![7], vec![9], vec![6]];
        let res = vec![7, 9, 6];
        assert_eq!(Solution::spiral_order(matrix), res);
    }
}
