pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
    fn dfs(nums: Vec<i32>, ans: &mut Vec<Vec<i32>>, buffer: &mut Vec<i32>) {
        if nums.is_empty() {
            ans.push(buffer.to_vec());
        }
        for (i, &value) in nums.iter().enumerate() {
            buffer.push(value);
            let mut tmp = nums.clone();
            tmp.remove(i);
            dfs(tmp, ans, buffer);
            buffer.pop();
        }
    }
    let mut ans = Vec::new();
    let mut p = Vec::new();
    dfs(nums, &mut ans, &mut p);
    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let nums = vec![1, 2, 3];
        let output = vec![
            vec![1, 2, 3],
            vec![1, 3, 2],
            vec![2, 1, 3],
            vec![2, 3, 1],
            vec![3, 1, 2],
            vec![3, 2, 1],
        ];
        assert_eq!(permute(nums), output);
    }

    #[test]
    fn test_case_2() {
        let nums = vec![0, 1];
        let output = vec![vec![0, 1], vec![1, 0]];
        assert_eq!(permute(nums), output);
    }

    #[test]
    fn test_case_3() {
        let nums = vec![1];
        let output = vec![vec![1]];
        assert_eq!(permute(nums), output);
    }
}
