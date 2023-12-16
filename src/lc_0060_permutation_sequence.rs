use crate::Solution;

impl Solution {
    pub fn get_permutation(n: i32, k: i32) -> String {
        let mut result = String::new();
        let mut k = k - 1;
        let mut factorial = vec![1; n as usize];

        for i in 1..(n as usize) {
            factorial[i] = factorial[i - 1] * i as i32;
        }

        let mut available = (1..=n).collect::<Vec<_>>();

        for i in (0..n as usize).rev() {
            let index = (k / factorial[i]) as usize;
            let num = available.remove(index);
            result.push_str(&num.to_string());
            k %= factorial[i];
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let n = 3;
        let k = 3;
        assert_eq!(Solution::get_permutation(n, k), "213");
    }

    #[test]
    fn test_case_2() {
        let n = 4;
        let k = 9;
        assert_eq!(Solution::get_permutation(n, k), "2314");
    }
    #[test]
    fn test_case_3() {
        let n = 3;
        let k = 1;
        assert_eq!(Solution::get_permutation(n, k), "123");
    }
}

