use crate::Solution;

impl Solution {
    pub fn min_operations_crawer(logs: Vec<String>) -> i32 {
        logs.iter()
            .filter(|op| *op != "./")
            .map(|op| if op == "../" { -1 } else { 1 })
            .fold(0_i32, |depth, crement| 0.max(depth + crement))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let logs = vec![
            "d1/".to_string(),
            "d2/".to_string(),
            "../".to_string(),
            "d21/".to_string(),
            "./".to_string(),
        ];
        assert_eq!(Solution::min_operations_crawer(logs), 2);
    }

    #[test]
    fn test_case_2() {
        let logs = vec![
            "d1/".to_string(),
            "d2/".to_string(),
            "./".to_string(),
            "d3/".to_string(),
            "../".to_string(),
            "d31/".to_string(),
            "./".to_string(),
        ];
        assert_eq!(Solution::min_operations_crawer(logs), 3);
    }

    #[test]
    fn test_case_3() {
        let logs = vec![
            "d1".to_string(),
            "../".to_string(),
            "../".to_string(),
            "../".to_string(),
        ];
        assert_eq!(Solution::min_operations_crawer(logs), 0);
    }
}
