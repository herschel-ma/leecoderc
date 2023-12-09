pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {
    solve_sudoku_helper(board, 0);
}

fn solve_sudoku_helper(board: &mut Vec<Vec<char>>, n: usize) -> bool {
    if n == 81 {
        return true;
    }
    let (i, j) = (n / 9, n % 9);
    if board[i][j] != '.' {
        return solve_sudoku_helper(board, n + 1);
    }

    let mask = check(board, i, j);
    for b in 1..=9 {
        if (mask >> b) & 1 == 1 {
            continue;
        }
        board[i][j] = std::char::from_digit(b, 10).unwrap();
        if solve_sudoku_helper(board, n + 1) {
            return true;
        }
        board[i][j] = '.'
    }
    false
}

fn check(board: &[Vec<char>], i: usize, j: usize) -> u16 {
    let mut res: u16 = 0;
    for n in 0..9 {
        let rc = board[i][n];
        let cc = board[n][j];
        let sc = board[i / 3 * 3 + n / 3][(j / 3 * 3) + (n % 3)];
        mask(&mut res, rc);
        mask(&mut res, cc);
        mask(&mut res, sc);
    }
    res
}

#[inline]
fn mask(x: &mut u16, c: char) {
    match c {
        '.' => {}
        _ => {
            *x |= 1 << c.to_digit(10).unwrap();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_sudoku() {
        let mut board = vec![
            vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
            vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
        ];
        let output = vec![
            vec!['5', '3', '4', '6', '7', '8', '9', '1', '2'],
            vec!['6', '7', '2', '1', '9', '5', '3', '4', '8'],
            vec!['1', '9', '8', '3', '4', '2', '5', '6', '7'],
            vec!['8', '5', '9', '7', '6', '1', '4', '2', '3'],
            vec!['4', '2', '6', '8', '5', '3', '7', '9', '1'],
            vec!['7', '1', '3', '9', '2', '4', '8', '5', '6'],
            vec!['9', '6', '1', '5', '3', '7', '2', '8', '4'],
            vec!['2', '8', '7', '4', '1', '9', '6', '3', '5'],
            vec!['3', '4', '5', '2', '8', '6', '1', '7', '9'],
        ];
        solve_sudoku(&mut board);
        assert_eq!(board, output);
    }
}
