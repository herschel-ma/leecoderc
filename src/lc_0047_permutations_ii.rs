pub fn permute_unique(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut res: Vec<Vec<i32>> = Vec::new();
    let mut cans: Vec<i32> = Vec::new();
    let mut nums = nums;
    let mut visited: Vec<bool> = vec![false; nums.len()];
    nums.sort_unstable();
    dfs_helper(nums, &mut cans, &mut res, &mut visited);

    fn dfs_helper(
        nums: Vec<i32>,
        cans: &mut Vec<i32>,
        res: &mut Vec<Vec<i32>>,
        visited: &mut Vec<bool>,
    ) {
        if cans.len() == nums.len() {
            res.push(cans.clone());
            return;
        }
        for (idx, &value) in nums.iter().enumerate() {
            // check dup entry
            if visited[idx] || idx > 0 && nums[idx] == nums[idx - 1] && !visited[idx - 1] {
                continue;
            }
            visited[idx] = true;
            cans.push(value);
            visited[idx] = true;
            dfs_helper(nums.clone(), cans, res, visited);
            cans.pop();
            visited[idx] = false;
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let nums = vec![1, 1, 2];
        let output = vec![vec![1, 1, 2], vec![1, 2, 1], vec![2, 1, 1]];
        assert_eq!(permute_unique(nums), output);
    }
    #[test]
    fn test_case_2() {
        let nums = vec![1, 2, 3];
        let output = vec![
            vec![1, 2, 3],
            vec![1, 3, 2],
            vec![2, 1, 3],
            vec![2, 3, 1],
            vec![3, 1, 2],
            vec![3, 2, 1],
        ];
        assert_eq!(permute_unique(nums), output);
    }
}
