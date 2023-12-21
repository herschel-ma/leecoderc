use crate::Solution;

impl Solution {
    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        let m = board.len();
        let n = board[0].len();
        let word = word.as_bytes();
        let mut vis = vec![vec![false; n]; m];

        fn dfs(
            i: usize,
            j: usize,
            c: usize,
            word: &[u8],
            vis: &mut [Vec<bool>],
            board: &[Vec<char>],
        ) -> bool {
            if board[i][j] as u8 != word[c] {
                return false;
            }

            if c == word.len() - 1 {
                return true;
            };
            vis[i][j] = true;

            let dirs = [[-1, 0], [0, -1], [1, 0], [0, 1]];
            for [x, y] in dirs.into_iter() {
                let i = x + i as i32;
                let j = y + j as i32;
                if i < 0 || i == board.len() as i32 || j < 0 || j == board[0].len() as i32 {
                    continue;
                }
                let (i, j) = (i as usize, j as usize);
                if !vis[i][j] && dfs(i, j, c + 1, word, vis, board) {
                    return true;
                }
            }

            vis[i][j] = false;
            false
        }

        for i in 0..m {
            for j in 0..n {
                if dfs(i, j, 0, word, &mut vis, &board) {
                    return true;
                }
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let board = vec![
            vec!["A", "B", "C", "E"],
            vec!["S", "F", "C", "S"],
            vec!["A", "D", "E", "E"],
        ];
        // convert board to Vec<Vec<char>>
        let vec_board = board
            .into_iter()
            .map(|inner_vec| {
                inner_vec
                    .into_iter()
                    .flat_map(|s| s.chars())
                    .collect::<Vec<char>>()
            })
            .collect();

        let word = String::from("ABCCED");
        assert!(Solution::exist(vec_board, word));
    }

    #[test]
    fn tsest_case_2() {
        let board = vec![
            vec!["A", "B", "C", "E"],
            vec!["S", "F", "C", "S"],
            vec!["A", "D", "E", "E"],
        ];
        let vec_board = board
            .into_iter()
            .map(|row| {
                row.into_iter()
                    .flat_map(|s| s.chars())
                    .collect::<Vec<char>>()
            })
            .collect();

        let word = String::from("SEE");
        assert!(Solution::exist(vec_board, word));
    }

    #[test]
    fn test_case_3() {
        let board = vec![
            vec!["A", "B", "C", "E"],
            vec!["S", "F", "C", "S"],
            vec!["A", "D", "E", "E"],
        ];
        let vec_board = board
            .into_iter()
            .map(|row| {
                row.into_iter()
                    .flat_map(|s| s.chars())
                    .collect::<Vec<char>>()
            })
            .collect();
        let word = String::from("ABCB");
        assert!(!Solution::exist(vec_board, word));
    }
}
