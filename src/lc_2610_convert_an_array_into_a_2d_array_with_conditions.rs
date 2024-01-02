use crate::Solution;

impl Solution {
    pub fn find_matrix(nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.iter()
            .fold((vec![0; nums.len()], vec![]), |(mut v, mut ans), &x| {
                while v[(x - 1) as usize] >= ans.len() {
                    ans.push(Vec::new());
                }
                ans[v[(x - 1) as usize]].push(x);

                v[(x - 1) as usize] += 1;
                (v, ans)
            })
            .1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let nums = vec![1, 3, 4, 1, 2, 3, 1];
        let res = vec![vec![1, 3, 4, 2], vec![1, 3], vec![1]];
        let output = Solution::find_matrix(nums);

        assert_eq!(output, res);
    }

    #[test]
    fn test_case_2() {
        let nums = vec![1, 2, 3, 4];
        let res = vec![vec![1, 2, 3, 4]];
        let output = Solution::find_matrix(nums);

        assert_eq!(output.clone(), res.clone());
    }
}

