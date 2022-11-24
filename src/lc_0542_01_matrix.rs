pub struct Solution;

impl Solution {
    pub fn update_matrix(mat: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        use std::collections::HashSet;

        let height = mat.len();
        let wigth = mat[0].len();
        let max = (height + wigth) as i32;
        let mut output = vec![vec![0; wigth]; height];

        let mut hs = HashSet::new();

        for i in 0..height {
            for j in 0..wigth {
                if mat[i][j] == 0 {
                    output[i][j] = 0;
                } else {
                    output[i][j] = max;
                    hs.insert((i, j));
                }
            }
        }

        while !hs.is_empty() {
            let entries = hs.drain().collect::<Vec<(usize, usize)>>();

            for (row, col) in entries {
                let up = if row == 0 { None } else { Some((row - 1, col)) };
                let down = if row + 1 == height {
                    None
                } else {
                    Some((row + 1, col))
                };
                let left = if col == 0 { None } else { Some((row, col - 1)) };
                let right = if col + 1 == wigth {
                    None
                } else {
                    Some((row, col + 1))
                };

                let mut min = max;
                for dir in [up, down, left, right].iter().filter_map(|&x| x) {
                    min = min.min(output[dir.0][dir.1]);
                }
                min += 1;

                if min < output[row][col] {
                    output[row][col] = min;
                    for dir in [up, down, left, right].iter().filter_map(|&x| x) {
                        hs.insert(dir);
                    }
                }
            }
        }
        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn ex1() {
        assert_eq!(
            Solution::update_matrix(vec![vec![0, 0, 0], vec![0, 1, 0], vec![0, 0, 0]]),
            vec![vec![0, 0, 0], vec![0, 1, 0], vec![0, 0, 0]]
        );
    }
    #[test]
    fn ex2() {
        assert_eq!(
            Solution::update_matrix(vec![vec![0, 0, 0], vec![0, 1, 0], vec![1, 1, 1]]),
            vec![vec![0, 0, 0], vec![0, 1, 0], vec![1, 2, 1]]
        );
    }
}
