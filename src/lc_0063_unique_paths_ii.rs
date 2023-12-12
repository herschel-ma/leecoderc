pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
    let m = obstacle_grid.len();
    let n = obstacle_grid[0].len();
    let mut dp = vec![vec![0; n]; m];
    if obstacle_grid[0][0] == 1 {
        return 0;
    }
    for j in 0..n {
        if obstacle_grid[0][j] == 1 {
            break;
        }
        dp[0][j] = 1;
    }
    for i in 0..m {
        if obstacle_grid[i][0] == 1 {
            break;
        }
        dp[i][0] = 1;
    }
    for i in 1..m {
        for j in 1..n {
            if obstacle_grid[i][j] == 1 {
                continue;
            }
            dp[i][j] = dp[i - 1][j] + dp[i][j - 1];
        }
    }
    dp[m - 1][n - 1]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let obstacle_grid = vec![vec![0, 0, 0], vec![0, 1, 0], vec![0, 0, 0]];
        let res = 2;
        assert_eq!(unique_paths_with_obstacles(obstacle_grid), res);
    }

    #[test]
    fn test_case_2() {
        let obstacle_grid = vec![vec![0, 1], vec![0, 0]];
        let res = 1;
        assert_eq!(unique_paths_with_obstacles(obstacle_grid), res);
    }
    #[test]
    fn test_wrong_answer() {
        let obstacle_grid = vec![vec![0, 0], vec![1, 1], vec![0, 0]];
        let res = 0;
        assert_eq!(unique_paths_with_obstacles(obstacle_grid), res);
    }
}
