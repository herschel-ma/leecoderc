use crate::Solution;
impl Solution {
    pub fn is_number(s: String) -> bool {
        let mut i = 0;
        let n = s.len();

        if let Some(c) = s.chars().nth(i) {
            if c == '+' || c == '-' {
                i += 1;
                if i == n {
                    return false;
                }
            }
        }
        if let Some(x) = s.chars().nth(i) {
            if x == '.'
                && (i + 1 == n
                    || if let Some(m) = s.chars().nth(i + 1) {
                        m == 'e' || m == 'E'
                    } else {
                        false
                    })
            {
                return false;
            }
        }

        let mut dot = 0;
        let mut e = 0;
        let mut j = i;

        while j < n {
            if let Some(c) = s.chars().nth(j) {
                if c == '.' {
                    if e > 0 || dot > 0 {
                        return false;
                    }
                    dot += 1;
                } else if c == 'e' || c == 'E' {
                    if e > 0 || j == i || j == n - 1 {
                        return false;
                    }
                    e += 1;
                    if let Some(x) = s.chars().nth(j + 1) {
                        if x == '+' || x == '-' {
                            j += 1;
                            if j == n - 1 {
                                return false;
                            }
                        }
                    }
                } else if !c.is_ascii_digit() {
                    return false;
                }
            }
            j += 1;
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let s = "0".to_string();
        assert!(Solution::is_number(s));
    }

    #[test]
    fn teset_case_2() {
        let s = "e".to_string();
        assert!(!Solution::is_number(s));
    }

    #[test]
    fn test_case_3() {
        let s = ".".to_string();
        assert!(!Solution::is_number(s));
    }
    #[test]
    fn test_dot_1() {
        let s = ".1".to_string();
        assert!(Solution::is_number(s));
    }
    #[test]
    fn test_wrong_answer() {
        let s = "+.".to_string();
        assert!(!Solution::is_number(s));
    }
    #[test]
    fn another_wrong_answer() {
        let s = "005047e+6".to_string();
        assert!(Solution::is_number(s));
    }
    #[test]
    fn test_case_4() {
        let s = "4e+".to_string();
        assert!(!Solution::is_number(s));
    }
}
