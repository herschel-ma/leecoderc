use std::collections::HashMap;

pub fn is_valid_1(s: String) -> bool {
    let mut stack = "".to_string();
    for ch in s.chars() {
        match ch {
            '(' => stack.push(')'),
            '{' => stack.push('}'),
            '[' => stack.push(']'),
            ')' | '}' | ']' => {
                if let Some(s) = stack.pop() {
                    if s != ch {
                        return false;
                    }
                } else {
                    return false;
                }
            }
            _ => return false,
        }
    }
    stack.is_empty()
}

pub fn is_valid(s: String) -> bool {
    let mut map = HashMap::new();
    map.insert('(', ')');
    map.insert('[', ']');
    map.insert('{', '}');

    let mut stack = vec![];
    for ch in s.chars() {
        if map.contains_key(&ch) {
            stack.push(map[&ch]);
        } else if stack.pop().unwrap_or('-') != ch {
            return false;
        }
    }
    stack.is_empty()
}

mod tests {
    #[test]
    fn ex1() {
        use super::*;
        assert!(is_valid("()".to_string()))
    }

    #[test]
    fn ex2() {
        use super::*;
        assert!(is_valid("()[]{}".to_string()))
    }

    #[test]
    fn ex3() {
        use super::*;
        assert!(!is_valid("(]".to_string()))
    }
}
