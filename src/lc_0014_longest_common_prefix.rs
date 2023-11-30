pub fn longest_common_prefix(strs: Vec<String>) -> String {
    let len = strs.iter().map(|e| e.len()).min().unwrap();
    for i in (1..=len).rev() {
        let target = strs[0][0..i].to_string();
        if strs.iter().all(|s| target == s[0..i]) {
            return target;
        }
    }
    String::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_equal() {
        let strs = vec![
            "flower".to_string(),
            "flow".to_string(),
            "flight".to_string(),
        ];
        let output = "fl".to_string();
        assert_eq!(longest_common_prefix(strs), output);
    }

    #[test]
    fn should_none() {
        let strs = vec!["dog".to_string(), "racecar".to_string(), "car".to_string()];
        let output = String::new();
        assert_eq!(longest_common_prefix(strs), output);
    }
}
