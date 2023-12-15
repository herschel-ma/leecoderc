pub fn total_n_queens(n: i32) -> i32 {
    let mut ans: i32 = 0;
    dfs(
        0,
        n,
        &mut ans,
        &mut vec![false; n as usize],
        &mut vec![false; (2 * n) as usize],
        &mut vec![false; (2 * n) as usize],
    );
    ans
}

fn dfs(
    i: i32,
    n: i32,
    ans: &mut i32,
    col: &mut [bool],
    pos_dia: &mut [bool],
    neg_dia: &mut [bool],
) {
    if i == n {
        *ans += 1;
        return;
    }
    for c in 0..n {
        if col[c as usize] || pos_dia[(i + c) as usize] || neg_dia[(i - c + n) as usize] {
            continue;
        }
        col[c as usize] = true;
        pos_dia[(i + c) as usize] = true;
        neg_dia[(i - c + n) as usize] = true;
        dfs(i + 1, n, ans, col, pos_dia, neg_dia);

        col[c as usize] = false;
        pos_dia[(i + c) as usize] = false;
        neg_dia[(i - c + n) as usize] = false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let n = 4;
        assert_eq!(total_n_queens(n), 2);
    }
    #[test]
    fn test_case_2() {
        let n = 1;
        assert_eq!(total_n_queens(n), 1);
    }
}
