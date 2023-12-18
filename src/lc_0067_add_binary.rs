use crate::Solution;
use std::cmp::Ordering;

impl Solution {
    pub fn add_binary_1(a: String, b: String) -> String {
        let a: Vec<i32> = a
            .chars()
            .rev()
            .map(|c| match c {
                '0' => 0,
                '1' => 1,
                _ => panic!("oops"),
            })
            .collect();
        let b: Vec<i32> = b
            .chars()
            .rev()
            .map(|c| match c {
                '0' => 0,
                '1' => 1,
                _ => panic!("oops"),
            })
            .collect();

        let mut i = 0;
        let mut carry = 0;
        let mut result = Vec::new();
        while i < a.len() && i < b.len() {
            let sum = a[i] + b[i] + carry;
            result.push(sum & 1);
            carry = (sum & 2) >> 1;
            i += 1;
        }
        while i < a.len() {
            let sum = a[i] + carry;
            result.push(sum & 1);
            carry = (sum & 2) >> 1;
            i += 1;
        }
        while i < b.len() {
            let sum = b[i] + carry;
            result.push(sum & 1);
            carry = (sum & 2) >> 1;
            i += 1;
        }

        if carry == 1 {
            result.push(carry);
        }

        let mut answer = "".to_string();
        while let Some(x) = result.pop() {
            if x == 1 {
                answer.push('1');
            } else {
                answer.push('0');
            }
        }

        answer
    }

    pub fn add_binary(a: String, b: String) -> String {
        let mut i = (a.len() as i32) - 1;
        let mut j = (b.len() as i32) - 1;
        let mut carry = 0;
        let mut ans = String::new();
        let a = a.as_bytes();
        let b = b.as_bytes();

        while i >= 0 || j >= 0 || carry > 0 {
            if i >= 0 {
                carry += a[i as usize] - b'0';
                i -= 1;
            }
            if j >= 0 {
                carry += b[j as usize] - b'0';
                j -= 1;
            }
            ans.push_str(&(carry & 1).to_string());
            carry >>= 1;
        }
        ans.chars().rev().collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(
            Solution::add_binary("11".to_string(), "1".to_string()),
            "100".to_string()
        );
    }

    #[test]
    fn ex2() {
        assert_eq!(
            Solution::add_binary("1010".to_string(), "1011".to_string()),
            "10101".to_string()
        );
    }

    #[test]
    fn ex3() {
        assert_eq!(
            Solution::add_binary("0".to_string(), "0".to_string()),
            "0".to_string()
        );
    }
    #[test]
    fn ex4() {
        assert_eq!(
            Solution::add_binary("1".to_string(), "111".to_string()),
            "1000".to_string()
        );
    }
    #[test]
    fn ex5() {
        assert_eq!(
            Solution::add_binary("1111".to_string(), "1111".to_string()),
            "11110".to_string()
        );
    }
}
