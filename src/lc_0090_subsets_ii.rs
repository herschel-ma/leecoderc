use crate::Solution;

impl Solution {
    pub fn subsets_with_dup(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut nums = nums;
        nums.sort();
        let mut res = vec![];
        fn dfs(i: usize, res: &mut Vec<Vec<i32>>, nums: &[i32], buf: &mut Vec<i32>) {
            for j in i..nums.len() {
                buf.push(nums[j]);
                if !res.contains(&buf.clone()) {
                    res.push(buf.clone());
                }
                dfs(j + 1, res, nums, buf);
                buf.pop();
            }
        }

        dfs(0, &mut res, &nums, &mut vec![]);

        res.insert(0, vec![]);
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let nums = vec![1, 2, 2];
        let mut output = vec![
            vec![],
            vec![1],
            vec![1, 2],
            vec![1, 2, 2],
            vec![2],
            vec![2, 2],
        ];
        let mut res = Solution::subsets_with_dup(nums);
        res.sort_unstable();
        output.sort_unstable();
        assert_eq!(res, output);
    }

    #[test]
    fn test_case_2() {
        let nums = vec![0];
        let mut output = vec![vec![], vec![0]];
        let mut res = Solution::subsets_with_dup(nums);
        res.sort_unstable();
        output.sort_unstable();
        assert_eq!(res, output);
    }
}
