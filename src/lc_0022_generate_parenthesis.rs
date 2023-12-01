pub fn generate_parenthesis(n: i32) -> Vec<String> {
    fn dfs(l: i32, r: i32, s: &mut String, res: &mut Vec<String>) {
        if l == 0 && r == 0 {
            res.push(s.clone());
            return;
        }
        if l > 0 {
            s.push('(');
            dfs(l - 1, r, s, res);
            s.pop();
        }
        if r > l {
            s.push(')');
            dfs(l, r - 1, s, res);
            s.pop();
        }
    }
    let mut res = Vec::new();
    dfs(n, n, &mut String::new(), &mut res);
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case1() {
        let n = 3;
        let output = vec![
            "((()))".to_string(),
            "(()())".to_string(),
            "(())()".to_string(),
            "()(())".to_string(),
            "()()()".to_string(),
        ];
        assert_eq!(generate_parenthesis(n), output)
    }

    #[test]
    fn test_case2() {
        let n = 1;
        let output = vec!["()".to_string()];
        assert_eq!(generate_parenthesis(n), output)
    }
}
