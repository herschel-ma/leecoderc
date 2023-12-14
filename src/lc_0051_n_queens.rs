use std::collections::HashSet;

pub fn n_queen(n: i32) -> Vec<Vec<String>> {
    let mut col: HashSet<i32> = HashSet::new();
    let mut pos_diag: HashSet<i32> = HashSet::new(); // r + c
    let mut neg_diag: HashSet<i32> = HashSet::new(); // r - c
    let mut res = vec![];
    let mut board = vec![vec!['.'; n as usize]; n as usize];

    fn dfs(
        r: i32,
        n: i32,
        res: &mut Vec<Vec<String>>,
        board: &mut Vec<Vec<char>>,
        col: &mut HashSet<i32>,
        pos_diag: &mut HashSet<i32>,
        neg_diag: &mut HashSet<i32>,
    ) {
        if r == n {
            let solution: Vec<String> = board
                .clone()
                .iter()
                .map(|row| row.iter().collect())
                .collect();
            res.push(solution);
            return;
        }
        for c in 0..n {
            if col.contains(&c) || pos_diag.contains(&(r + c)) || neg_diag.contains(&(r - c)) {
                continue;
            }
            col.insert(c);
            pos_diag.insert(r + c);
            neg_diag.insert(r - c);
            board[r as usize][c as usize] = 'Q';
            dfs(r + 1, n, res, board, col, pos_diag, neg_diag);
            col.remove(&c);
            pos_diag.remove(&(r + c));
            neg_diag.remove(&(r - c));
            board[r as usize][c as usize] = '.';
        }
    }

    dfs(
        0,
        n,
        &mut res,
        &mut board,
        &mut col,
        &mut pos_diag,
        &mut neg_diag,
    );

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let n = 4;
        let output = vec![
            vec![
                ".Q..".to_string(),
                "...Q".to_string(),
                "Q...".to_string(),
                "..Q.".to_string(),
            ],
            vec![
                "..Q.".to_string(),
                "Q...".to_string(),
                "...Q".to_string(),
                ".Q..".to_string(),
            ],
        ];
        assert_eq!(n_queen(n), output);
    }

    #[test]
    fn test_case_2() {
        let n = 1;
        let output = vec![vec!["Q".to_string()]];
        assert_eq!(n_queen(n), output);
    }
}
