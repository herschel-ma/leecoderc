use crate::Solution;
impl Solution {
    pub fn get_lucky(s: String, k: i32) -> i32 {
        let mut s = s
            .chars()
            .map(|c| (c as u8 - b'a' + 1).to_string())
            .collect::<Vec<String>>()
            .join("");

        for _ in 0..k {
            s = s
                .chars()
                .map(|c| c as i32 - '0' as i32)
                .sum::<i32>()
                .to_string();
        }

        s.parse::<i32>().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let s = "iiii".to_string();
        assert_eq!(Solution::get_lucky(s, 1), 36);
    }

    #[test]
    fn test_case_2() {
        let s = "leetcode".to_string();
        assert_eq!(Solution::get_lucky(s, 2), 6);
    }

    #[test]
    fn test_case_3() {
        let s = "zbax".to_string();
        assert_eq!(Solution::get_lucky(s, 2), 8);
    }
}
