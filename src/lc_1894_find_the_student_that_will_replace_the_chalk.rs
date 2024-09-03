use crate::Solution;

impl Solution {
    pub fn chalk_replacer(chalk: Vec<i32>, k: i32) -> i32 {
        let total_sum = chalk.iter().map(|&x| x as i64).sum::<i64>();
        let mut remaining_chalk = (k as i64) % total_sum;
        for (student_idx, &student_chalk_use) in chalk.iter().enumerate() {
            if remaining_chalk < (student_chalk_use as i64) {
                return student_idx as i32;
            }
            remaining_chalk -= student_chalk_use as i64;
        }
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let chalk = vec![5, 1, 5];
        assert_eq!(Solution::chalk_replacer(chalk, 22), 0);
    }

    #[test]
    fn test_case_2() {
        let chalk = vec![3, 4, 1, 2];
        assert_eq!(Solution::chalk_replacer(chalk, 25), 1);
    }
}
