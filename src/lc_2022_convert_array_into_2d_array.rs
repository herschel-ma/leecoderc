use crate::Solution;

impl Solution {
    pub fn construct_2d_array(original: Vec<i32>, m: i32, n: i32) -> Vec<Vec<i32>> {
        if original.len() != m as usize * n as usize {
            return vec![];
        }
        let mut ans = Vec::with_capacity(m as usize);
        for i in 0..m {
            let start = i as usize * n as usize;
            let end = (i + 1) as usize * n as usize;
            ans.push(original[start..end].to_vec());
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let original = vec![1, 2, 3, 4];
        assert_eq!(
            Solution::construct_2d_array(original, 2, 2),
            vec![vec![1, 2], vec![3, 4]]
        );
    }

    #[test]
    fn test_case_2() {
        let original = vec![1, 2, 3];
        assert_eq!(
            Solution::construct_2d_array(original, 1, 3),
            vec![vec![1, 2, 3]]
        );
    }

    #[test]
    fn test_case_() {
        let original = vec![1, 2];
        let ans: Vec<Vec<i32>> = Vec::new();
        assert_eq!(Solution::construct_2d_array(original, 1, 1), ans);
    }
}
