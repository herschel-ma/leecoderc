use crate::Solution;

impl Solution {
    pub fn find_judge(n: i32, trust: Vec<Vec<i32>>) -> i32 {
        let mut cnt1: Vec<i32> = vec![0; (n + 1) as usize];
        let mut cnt2: Vec<i32> = vec![0; (n + 1) as usize];
        trust.iter().for_each(|x| {
            cnt1[x[0] as usize] += 1;
            cnt2[x[1] as usize] += 1;
        });
        for i in 1..=n as usize {
            if cnt1[i] == 0 && cnt2[i] == n - 1 {
                return i as i32;
            }
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let n = 2;
        let trust = vec![vec![1, 2]];
        assert_eq!(Solution::find_judge(n, trust), 2);
    }

    #[test]
    fn test_case_2() {
        let n = 3;
        let trust = vec![vec![1, 3], vec![2, 3]];
        assert_eq!(Solution::find_judge(n, trust), 3);
    }

    #[test]
    fn test_case_3() {
        let n = 3;
        let trust = vec![vec![1, 3], vec![2, 3], vec![3, 1]];
        assert_eq!(Solution::find_judge(n, trust), -1);
    }
}
