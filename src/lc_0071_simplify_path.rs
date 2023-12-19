use crate::Solution;

impl Solution {
    pub fn simplify_path(path: String) -> String {
        let mut stack: Vec<&str> = vec![];
        let vec: Vec<&str> = path.split('/').collect();
        for v in vec {
            match v {
                "." | "" => continue,
                ".." => {
                    if !stack.is_empty() {
                        stack.pop();
                    }
                }
                _ => stack.push(v),
            }
        }
        "/".to_string() + stack.join("/").as_str()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let path = String::from("/home/");
        let res = String::from("/home");
        assert_eq!(Solution::simplify_path(path), res);
    }

    #[test]
    fn test_case_2() {
        let path = String::from("/../");
        let res = String::from("/");
        assert_eq!(Solution::simplify_path(path), res);
    }

    #[test]
    fn test_case_3() {
        let path = String::from("/home//foo/");
        let res = String::from("/home/foo");
        assert_eq!(Solution::simplify_path(path), res);
    }

    #[test]
    fn test_case_4() {
        let path = String::from("/a/./b/../../c/");
        let res = String::from("/c");
        assert_eq!(Solution::simplify_path(path), res);
    }
}
