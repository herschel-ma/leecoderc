use crate::Solution;

impl Solution {
    pub fn stone_game_ii(piles: Vec<i32>) -> i32 {
        let n = piles.len();
        let mut s = vec![0; n + 1];
        let mut memo = vec![vec![-1; n + 1]; n];

        for i in 0..n {
            s[i + 1] = s[i] + piles[i];
        }

        fn dfs(i: usize, m: usize, n: usize, s: &[i32], memo: &mut Vec<Vec<i32>>) -> i32 {
            if m * 2 >= n - i {
                return s[n] - s[i];
            }
            if memo[i][m] != -1 {
                return memo[i][m];
            }
            let mut ans = i32::MIN;
            for x in 1..=2 * m {
                ans = ans.max(s[n] - s[i] - dfs(i + x, m.max(x), n, s, memo));
            }
            memo[i][m] = ans;
            memo[i][m]
        }

        dfs(0, 1, n, &s, &mut memo)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let piles = vec![2, 7, 9, 4, 4];
        assert_eq!(Solution::stone_game_ii(piles), 10);
    }

    #[test]
    fn test_case_2() {
        let piles = vec![1, 2, 3, 4, 5, 100];
        assert_eq!(Solution::stone_game_ii(piles), 104);
    }
}
