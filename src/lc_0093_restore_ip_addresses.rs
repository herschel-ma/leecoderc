use crate::Solution;

impl Solution {
    pub fn restore_ip_addresses(s: String) -> Vec<String> {
        let mut ans = Vec::new();

        fn backtrack<'a>(ans: &mut Vec<String>, s: &'a [u8], cur: &mut Vec<&'a [u8]>, i: usize) {
            if i == s.len() && cur.len() == 4 {
                ans.push(String::from_utf8(cur.join(&b'.')).unwrap());
            }

            if i == s.len() || cur.len() == 4 {
                return;
            }

            let is_valid = |s: &[u8]| {
                if s.len() > 1 && s[0] == b'0' {
                    return false;
                }
                let oct = s.iter().fold(0u16, |acc, &d| acc * 10 + (d - b'0') as u16);
                oct < 256
            };

            for len in 1..=3.min(s.len() - i) {
                let slice = &s[i..i + len];
                if !is_valid(slice) {
                    continue;
                }

                cur.push(slice);
                backtrack(ans, s, cur, i + len);
                cur.pop();
            }
        }

        backtrack(&mut ans, s.as_bytes(), &mut Vec::new(), 0);
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let s = String::from("25525511135");
        let res = [
            String::from("255.255.111.35"),
            String::from("255.255.11.135"),
        ];
        let ans = Solution::restore_ip_addresses(s);
        res.iter().for_each(|e| assert!(ans.contains(e)));
    }

    #[test]
    fn test_case_2() {
        let s = String::from("0000");
        let res = [String::from("0.0.0.0")];
        let ans = Solution::restore_ip_addresses(s);
        res.iter().for_each(|e| assert!(ans.contains(e)));
    }

    #[test]
    fn test_case_3() {
        let s = String::from("101023");
        let res = [
            String::from("1.0.10.23"),
            String::from("1.0.102.3"),
            String::from("10.10.2.3"),
            String::from("101.0.2.3"),
        ];
        let ans = Solution::restore_ip_addresses(s);
        res.iter().for_each(|e| assert!(ans.contains(e)));
    }
}
