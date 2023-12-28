use crate::Solution;

struct Length(usize, usize);
struct Index(usize, usize);

impl Solution {
    pub fn is_interleave(s1: String, s2: String, s3: String) -> bool {
        let m = s1.len();
        let n = s2.len();
        if m + n != s3.len() {
            return false;
        }
        let mut memo = vec![vec![-1; n + 1]; m + 1];
        let s1 = s1.chars().collect();
        let s2 = s2.chars().collect();
        let s3 = s3.chars().collect();
        let len = Length(m, n);
        let mut index = Index(0, 0);

        fn helper(
            memo: &mut Vec<Vec<i32>>,
            len: &Length,
            index: &mut Index,
            s1: &Vec<char>,
            s2: &Vec<char>,
            s3: &Vec<char>,
        ) -> bool {
            let (m, n) = (len.0, len.1);
            let (i, j) = (index.0, index.1);
            if i >= m && j >= n {
                return true;
            }

            if memo[i][j] != -1 {
                return memo[i][j] == 1;
            }

            // Set the initial value
            memo[i][j] = 0;

            let k = i + j;

            if i < m && s1[i] == s3[k] && helper(memo, len, &mut Index(i + 1, j), s1, s2, s3) {
                memo[i][j] = 1;
            }

            if j < n && s2[j] == s3[k] && helper(memo, len, &mut Index(i, j + 1), s1, s2, s3) {
                memo[i][j] = 1;
            }
            memo[i][j] == 1
        }

        helper(&mut memo, &len, &mut index, &s1, &s2, &s3)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let s1 = String::from("aabcc");
        let s2 = String::from("dbbca");
        let s3 = String::from("aadbbcbcac");
        assert!(Solution::is_interleave(s1, s2, s3));
    }

    #[test]
    fn test_case_2() {
        let s1 = String::from("aabcc");
        let s2 = String::from("dbbca");
        let s3 = String::from("aadbbbaccc");
        assert!(!Solution::is_interleave(s1, s2, s3));
    }

    #[test]
    fn test_case_3() {
        let s1 = String::from("");
        let s2 = String::from("");
        let s3 = String::from("");
        assert!(Solution::is_interleave(s1, s2, s3))
    }
}

