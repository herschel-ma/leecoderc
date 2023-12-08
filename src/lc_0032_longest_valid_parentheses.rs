pub fn longest_valid_parentheses(s: String) -> i32 {
pub fn longest_valid_parentheses_stack(s: String) -> i32 {
    let mut stack = vec![-1];
    let mut res = 0;
    for i in 0..s.len() {
        if let Some('(') = s.chars().nth(i) {
            stack.push(i as i32);
        } else {
            stack.pop().unwrap();
            if stack.is_empty() {
                stack.push(i as i32);
            } else {
                res = std::cmp::max(res, i as i32 - stack.last().unwrap())
            }
        }
    }
    res
}

pub fn longest_valid_parentheses(s: String) -> i32 {
    // ()(())
    let mut max_length = 0;
    let mut dp = vec![0; s.len()];
    for i in 1..s.len() {
        if s.chars().nth(i).unwrap() == ')' {
            if s.chars().nth(i - 1).unwrap() == '(' {
                dp[i] = if i >= 2 { dp[i - 2] + 2 } else { 2 };
            } else if i as i32 - dp[i - 1] > 0
                && s.chars().nth(i - dp[i - 1] as usize - 1).unwrap() == '('
            {
                dp[i] = dp[i - 1]
                    + 2
                    + if i - dp[i - 1] as usize >= 2 {
                        dp[i - dp[i - 1] as usize - 2]
                    } else {
                        0
                    };
            }
            max_length = max_length.max(dp[i]);
        }
    }
    max_length
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let s = String::from("(()");
        let res = 2;
        assert_eq!(longest_valid_parentheses(s), res);
    }

    #[test]
    fn test_case_2() {
        let s = String::from(")()())");
        let res = 4;
        assert_eq!(longest_valid_parentheses(s), res)
    }

    #[test]
    fn test_wrong_answer() {
        let s = String::new();
        let res = 0;
        assert_eq!(longest_valid_parentheses(s), res)
    }
}

