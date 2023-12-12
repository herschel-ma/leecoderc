pub fn unique_paths(m: i32, n: i32) -> i32 {
    let m = m as usize;
    let n = n as usize;
    let mut dp: Vec<Vec<i32>> = vec![vec![0; n]; m];

    for r in 0..m {
        for c in 0..n {
            if r == 0 || c == 0 {
                dp[r][c] = 1;
            } else {
                dp[r][c] = dp[r][c - 1] + dp[r - 1][c];
            }
        }
    }
    dp[m - 1][n - 1]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let m = 3;
        let n = 7;
        let res = 28;
        assert_eq!(unique_paths(m, n), res);
    }

    #[test]
    fn test_case_2() {
        let m = 3;
        let n = 2;
        let res = 3;
        assert_eq!(unique_paths(m, n), res);
    }
}
