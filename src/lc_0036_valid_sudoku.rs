pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
    let mut rows = [0u16; 9];
    let mut cols = [0u16; 9];
    let mut boxes = [0u16; 9];
    for (i, row) in board.iter().enumerate() {
        for (j, &c) in row.iter().enumerate() {
            match c {
                '.' => continue,
                _ => {
                    let b: usize = 3 * (i / 3) + j / 3;
                    let cur = 1 << (c.to_digit(10).unwrap());
                    if rows[i] & cur != 0 || cols[j] & cur != 0 || boxes[b] & cur != 0 {
                        return false;
                    }

                    rows[i] |= cur;
                    cols[j] |= cur;
                    boxes[b] |= cur;
                }
            }
        }
    }
    true
}

pub fn convert_board(board: [[&str; 9]; 9]) -> Vec<Vec<char>> {
    Vec::from(board.map(|x| Vec::from(x.map(|s| s.chars().next().unwrap()))))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let board = [
            ["5", "3", ".", ".", "7", ".", ".", ".", "."],
            ["6", ".", ".", "1", "9", "5", ".", ".", "."],
            [".", "9", "8", ".", ".", ".", ".", "6", "."],
            ["8", ".", ".", ".", "6", ".", ".", ".", "3"],
            ["4", ".", ".", "8", ".", "3", ".", ".", "1"],
            ["7", ".", ".", ".", "2", ".", ".", ".", "6"],
            [".", "6", ".", ".", ".", ".", "2", "8", "."],
            [".", ".", ".", "4", "1", "9", ".", ".", "5"],
            [".", ".", ".", ".", "8", ".", ".", "7", "9"],
        ];
        assert!(is_valid_sudoku(convert_board(board)));
    }

    #[test]
    fn test_case_2() {
        let board = [
            ["8", "3", ".", ".", "7", ".", ".", ".", "."],
            ["6", ".", ".", "1", "9", "5", ".", ".", "."],
            [".", "9", "8", ".", ".", ".", ".", "6", "."],
            ["8", ".", ".", ".", "6", ".", ".", ".", "3"],
            ["4", ".", ".", "8", ".", "3", ".", ".", "1"],
            ["7", ".", ".", ".", "2", ".", ".", ".", "6"],
            [".", "6", ".", ".", ".", ".", "2", "8", "."],
            [".", ".", ".", "4", "1", "9", ".", ".", "5"],
            [".", ".", ".", ".", "8", ".", ".", "7", "9"],
        ];
        assert!(!is_valid_sudoku(convert_board(board)))
    }
}
