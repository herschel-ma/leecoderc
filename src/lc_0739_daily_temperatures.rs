use crate::Solution;

impl Solution {
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        let n = temperatures.len();
        let mut ans = vec![0; n];
        let mut stack: Vec<usize> = Vec::new();

        for i in (0..n).rev() {
            while !stack.is_empty() && temperatures[stack[stack.len() - 1]] <= temperatures[i] {
                stack.pop();
            }
            if !stack.is_empty() {
                ans[i] = (stack[stack.len() - 1] - i) as i32;
            }
            stack.push(i);
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let temperatures = vec![73, 74, 75, 71, 69, 72, 76, 73];
        assert_eq!(
            Solution::daily_temperatures(temperatures),
            vec![1, 1, 4, 2, 1, 1, 0, 0]
        );
    }

    #[test]
    fn test_case_2() {
        let temperatures = vec![30, 40, 50, 60];
        assert_eq!(Solution::daily_temperatures(temperatures), vec![1, 1, 1, 0]);
    }

    #[test]
    fn test_case_3() {
        let temperatures = vec![30, 60, 90];
        assert_eq!(Solution::daily_temperatures(temperatures), vec![1, 1, 0]);
    }
}
