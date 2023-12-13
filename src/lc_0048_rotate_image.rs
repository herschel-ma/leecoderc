pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
    let m = matrix.len();
    let n = matrix[0].len();
    // up and down swap
    for r in 0..(m >> 1) {
        for c in 0..n {
            let t = matrix[n - r - 1][c];
            matrix[n - r - 1][c] = matrix[r][c];
            matrix[r][c] = t;
        }
    }
    // diagonal swap
    for r in 0..m {
        for c in 0..r {
            let t = matrix[c][r];
            matrix[c][r] = matrix[r][c];
            matrix[r][c] = t;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let mut matrix = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let output = vec![vec![7, 4, 1], vec![8, 5, 2], vec![9, 6, 3]];
        rotate(&mut matrix);
        assert_eq!(matrix, output);
    }

    #[test]
    fn test_case_2() {
        let mut matrix = vec![
            vec![5, 1, 9, 11],
            vec![2, 4, 8, 10],
            vec![13, 3, 6, 7],
            vec![15, 14, 12, 16],
        ];
        let output = vec![
            vec![15, 13, 2, 5],
            vec![14, 3, 4, 1],
            vec![12, 6, 8, 9],
            vec![16, 7, 10, 11],
        ];
        rotate(&mut matrix);
        assert_eq!(matrix, output)
    }
}

