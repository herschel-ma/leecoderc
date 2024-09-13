use crate::Solution;

impl Solution {
    pub fn xor_queries(mut arr: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        for i in 1..arr.len() {
            arr[i] ^= arr[i - 1];
        }
        let mut ans = vec![];
        for query in queries {
            if query[0] > 0 {
                ans.push(arr[query[1] as usize] ^ arr[query[0] as usize - 1]);
            } else {
                ans.push(arr[query[1] as usize]);
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let arr = vec![1, 3, 4, 8];
        let queries = vec![vec![0, 1], vec![1, 2], vec![0, 3], vec![3, 3]];
        assert_eq!(Solution::xor_queries(arr, queries), vec![2, 7, 14, 8]);
    }

    #[test]
    fn test_case_2() {
        let arr = vec![4, 8, 2, 10];
        let queries = vec![vec![2, 3], vec![1, 3], vec![0, 0], vec![0, 3]];
        assert_eq!(Solution::xor_queries(arr, queries), vec![8, 0, 4, 4]);
    }
}
