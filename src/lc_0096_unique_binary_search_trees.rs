use crate::Solution;

impl Solution {
    pub fn num_trees(n: i32) -> i32 {
        let n = n as usize;
        let mut f = vec![0; n + 1];
        f[0] = 1;
        for i in 1..=n {
            for j in 0..i {
                f[i] += f[j] * f[i - j - 1];
            }
        }

        f[n] as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(Solution::num_trees(3), 5);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(Solution::num_trees(1), 1);
    }
}

