use crate::Solution;

impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        fn dfs(i: i32, res: &mut Vec<Vec<i32>>, buffer: &mut Vec<i32>, nums: &[i32]) {
            let n = nums.len();
            if n == 0 {
                res.push(vec![]);
            }
            res.push(buffer.clone());

            if i as usize > n {
                return;
            }
            for j in i as usize..n {
                buffer.push(nums[j]);
                dfs(j as i32 + 1, res, buffer, nums);
                buffer.pop();
            }
        }

        let mut res: Vec<Vec<i32>> = Vec::new();
        dfs(0, &mut res, &mut vec![], &nums);
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let nums = vec![1, 2, 3];
        let mut output = vec![
            vec![],
            vec![1],
            vec![2],
            vec![1, 2],
            vec![3],
            vec![1, 3],
            vec![2, 3],
            vec![1, 2, 3],
        ];
        let mut res = Solution::subsets(nums);
        res.sort();
        output.sort();
        assert_eq!(res, output);
    }

    #[test]
    fn test_case_2() {
        let nums = vec![0];
        let mut output = vec![vec![], vec![0]];
        let mut res = Solution::subsets(nums);
        res.sort();
        output.sort();
        assert_eq!(res, output);
    }
}
