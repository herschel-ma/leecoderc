use crate::Solution;
use std::collections::HashMap;

impl Solution {
    pub fn diff_ways_to_compute(expression: String) -> Vec<i32> {
        // 记忆化搜索
        // memorize
        let mut memo: HashMap<String, Vec<i32>> = HashMap::new();
        fn dfs(expression: &str, memo: &mut HashMap<String, Vec<i32>>) -> Vec<i32> {
            // memory cache
            if let Some(res) = memo.get(expression) {
                return res.clone();
            }
            // Base Case:
            if let Ok(n) = expression.parse::<i32>() {
                return vec![n];
            }

            let mut ans: Vec<i32> = Vec::new();
            // 遍历 expression 并找到递归分割点
            for (i, v) in expression.chars().enumerate() {
                if v == '+' || v == '-' || v == '*' {
                    // left sub-expression
                    let left = dfs(&expression[0..i], memo);
                    // right sub-expression
                    let right = dfs(&expression[i + 1..], memo);
                    // conbine results in left and right results.
                    for &l in left.iter() {
                        for &r in right.iter() {
                            match v {
                                '+' => ans.push(l + r),
                                '-' => ans.push(l - r),
                                '*' => ans.push(l * r),
                                _ => unreachable!(),
                            }
                        }
                    }
                }
            }
            // memorize
            memo.insert(expression.to_string(), ans.clone());
            ans
        }
        dfs(&expression, &mut memo)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let expression = "2-1-1".to_string();
        let mut expected = vec![0, 2];
        let mut res = Solution::diff_ways_to_compute(expression);
        expected.sort();
        res.sort();
        assert_eq!(res, expected);
    }

    #[test]
    fn test_case_2() {
        let expression = "2*3-4*5".to_string();
        let mut expected = vec![-34, -14, -10, -10, 10];
        let mut res = Solution::diff_ways_to_compute(expression);
        expected.sort();
        res.sort();
        assert_eq!(res, expected);
    }
}
