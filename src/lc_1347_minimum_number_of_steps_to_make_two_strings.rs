use crate::Solution;

impl Solution {
    pub fn min_steps(s: String, t: String) -> i32 {
        let mut counts = [0; 26];
        s.as_bytes()
            .iter()
            .zip(t.as_bytes().iter())
            .for_each(|(s, t)| {
                counts[(s - b'a') as usize] += 1;
                counts[(t - b'a') as usize] -= 1;
            });

        counts.into_iter().map(|count| count.max(0)).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_anagram_1() {
        let s = String::from("bab");
        let t = String::from("aba");
        assert_eq!(Solution::min_steps(s, t), 1);
    }

    #[test]
    fn test_anagram_2() {
        let s = String::from("leetcode");
        let t = String::from("practice");
        assert_eq!(Solution::min_steps(s, t), 5);
    }

    #[test]
    fn test_anagram_3() {
        let s = String::from("anagram");
        let t = String::from("mangaar");
        assert_eq!(Solution::min_steps(s, t), 0);
    }
}

