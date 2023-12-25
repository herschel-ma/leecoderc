use crate::Solution;

impl Solution {
    pub fn gray_code(n: i32) -> Vec<i32> {
        let mut res = vec![];
        for i in 0..(1 << n) {
            res.push(i ^ (i >> 1))
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let n = 2;
        let res_1 = vec![0, 1, 3, 2];
        let res_2 = vec![0, 2, 3, 1];
        assert!(Solution::gray_code(n) == res_1 || Solution::gray_code(n) == res_2);
    }

    #[test]
    fn test_case_2() {
        let n = 1;
        let res = vec![0, 1];
        assert_eq!(Solution::gray_code(n), res);
    }
}
