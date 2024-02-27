use crate::Solution;

impl Solution {
    pub fn decode_string(s: String) -> String {
        let mut current: String = String::new();
        let mut stack: Vec<(String, usize)> = Vec::new();
        let mut acc: usize = 0;

        for c in s.chars() {
            match c {
                '[' => {
                    stack.push((current.clone(), acc));
                    acc = 0;
                    current.clear();
                }
                ']' => {
                    let (pre, repeat) = stack.pop().unwrap();
                    current = pre + &current.repeat(repeat);
                }
                '0'..='9' => {
                    acc = acc * 10 + (c.to_digit(10).unwrap() as usize);
                }
                _ => {
                    current.push(c);
                }
            }
        }

        current
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let s = "3[a]2[bc]".to_string();
        let output = "aaabcbc".to_string();
        assert_eq!(Solution::decode_string(s), output);
    }

    #[test]
    fn test_case_2() {
        let s = "3[a2[c]]".to_string();
        let output = "accaccacc".to_string();
        assert_eq!(Solution::decode_string(s), output);
    }
    #[test]
    fn test_case_3() {
        let s = "2[abc]3[cd]ef".to_string();
        let output = "abcabccdcdcdef".to_string();
        assert_eq!(Solution::decode_string(s), output);
    }
}
