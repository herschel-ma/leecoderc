pub fn build_array(target: Vec<i32>, _n: i32) -> Vec<String> {
    let target_set: std::collections::HashSet<_> = target.iter().cloned().collect();
    let mut result = vec![];

    for i in 1..=*target.last().unwrap() {
        if target_set.contains(&i) {
            result.push("Push".to_string());
        } else {
            result.push("Push".to_string());
            result.push("Pop".to_string());
        }
    }
    result
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case1() {
        let target = vec![1, 3];
        let n = 3;
        let expected = vec![
            "Push".to_string(),
            "Push".to_string(),
            "Pop".to_string(),
            "Push".to_string(),
        ];
        assert_eq!(build_array(target, n), expected)
    }

    #[test]
    fn test_case2() {
        let target = vec![1, 2];
        let n = 4;
        let expected = vec!["Push".to_string(), "Push".to_string()];
        assert_eq!(build_array(target, n), expected)
    }
    #[test]
    fn test_case3() {
        let target = vec![1, 2, 3];
        let n = 3;
        let expected = vec!["Push".to_string(), "Push".to_string(), "Push".to_string()];
        assert_eq!(build_array(target, n), expected)
    }
}
