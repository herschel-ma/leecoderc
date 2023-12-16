use std::usize;

use crate::Solution;

impl Solution {
    pub fn get_permutation_2(n: i32, k: i32) -> String {
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
    pub fn get_permutation(n: i32, k: i32) -> String {
        let mut k = k;
        let mut result = String::new();
        let mut factorial = vec![1; n as usize];
        for i in 1..n as usize {
            factorial[i] = factorial[i - 1] * i as i32;
        }
        let mut used = vec![false; n as usize + 1];

        for i in 0..n as usize {
            let cnt = factorial[n as usize - i - 1];
            for j in 1..=n {
                if used[j as usize] {
                    continue;
                }
                if k > cnt {
                    k -= cnt
                } else {
                    result.push_str(&j.to_string());
                    used[j as usize] = true;
                    break;
                }
            }
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

