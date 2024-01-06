use crate::Solution;

impl Solution {
    pub fn job_scheduling(start_time: Vec<i32>, end_time: Vec<i32>, profit: Vec<i32>) -> i32 {
        let n = start_time.len();

        let mut tasks = (0..n)
            .map(|i| (start_time[i], end_time[i], profit[i]))
            .collect::<Vec<_>>();

        tasks.sort_unstable_by_key(|t| t.1);
        let mut dp = vec![0; n];
        dp[0] = tasks[0].2;

        for i in 1..n {
            let (start, _end, mut profit) = tasks[i];
            let j = tasks[1..i].partition_point(|&(_, end, _)| end <= start);

            if tasks[j].1 <= start {
                profit += dp[j];
            }

            dp[i] = dp[i - 1].max(profit);
        }

        dp[n - 1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(
            Solution::job_scheduling(vec![1, 2, 3, 3], vec![3, 4, 5, 6], vec![50, 10, 40, 70]),
            120
        );
    }

    #[test]
    fn test_case_2() {
        assert_eq!(
            Solution::job_scheduling(
                vec![1, 2, 3, 4, 6],
                vec![3, 5, 10, 6, 9],
                vec![20, 20, 100, 70, 60]
            ),
            150
        );
    }

    #[test]
    fn test_case_3() {
        assert_eq!(
            Solution::job_scheduling(vec![1, 1, 1], vec![2, 3, 4], vec![5, 6, 4]),
            6
        );
    }
}

