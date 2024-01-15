use std::collections::{HashMap, HashSet};

use crate::Solution;

impl Solution {
    /// You are given an integer array matches where `matches[i] = [winneri, loseri]`
    /// indicates that the player `winneri` defeated player `loseri` in a match.
    ///
    /// Return a list `answer` of size `2` where:
    ///     - `answer[0]` is a list of all players that have not lost any match.
    ///     - `answer[1]` is a list of all players that have lost exactly one match.
    /// The values in the two lists should be returned in `increasing` order.
    ///
    /// Note:
    ///     - You should only consider the players that have played `at least one` match.
    ///     - The test cases are generated such that `no` two matches will have the `same` outcome.
    ///
    /// https://leetcode.com/problems/find-players-with-zero-or-one-losses/
    pub fn find_winners(matches: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut res = vec![vec![]; 2];
        let mut counter = HashMap::new();
        let mut all = vec![];
        let mut losers = vec![];
        let mut lose_one = vec![];

        for mat in matches.iter() {
            all.push(mat[0]);
            all.push(mat[1]);
            losers.push(mat[1]);

            *counter.entry(mat[1]).or_insert(0) += 1;
            if counter[&mat[1]] == 1 {
                lose_one.push(mat[1]);
            } else {
                lose_one.retain(|&x| x != mat[1]);
            }
        }

        let all_set = all.into_iter().collect::<HashSet<i32>>();
        let losers_set = losers.into_iter().collect::<HashSet<i32>>();
        let mut winners = all_set
            .iter()
            .filter(|&x| !losers_set.contains(x))
            .collect::<Vec<_>>();

        winners.sort();
        lose_one.sort();

        res[0].extend(winners);
        res[1].extend(lose_one);

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let matches = vec![
            vec![1, 3],
            vec![2, 3],
            vec![3, 6],
            vec![5, 6],
            vec![5, 7],
            vec![4, 5],
            vec![4, 8],
            vec![4, 9],
            vec![10, 4],
            vec![10, 9],
        ];
        let output = vec![vec![1, 2, 10], vec![4, 5, 7, 8]];
        assert_eq!(Solution::find_winners(matches), output);
    }

    #[test]
    fn test_case_2() {
        let matches = vec![vec![2, 3], vec![1, 3], vec![5, 4], vec![6, 4]];
        let output = vec![vec![1, 2, 5, 6], vec![]];
        assert_eq!(Solution::find_winners(matches), output);
    }
}

