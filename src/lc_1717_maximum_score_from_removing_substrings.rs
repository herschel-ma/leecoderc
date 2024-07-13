use crate::Solution;

impl Solution {
    pub fn maximum_gain(s: String, x: i32, y: i32) -> i32 {
        fn remove(s: &mut Vec<u8>, a: &[u8], x: i32) -> i32 {
            let mut i = 0;
            let mut ans = 0;
            for j in 0..s.len() {
                s[i] = s[j];
                i += 1;
                if i > 1 && s[i - 2] == a[0] && s[i - 1] == a[1] {
                    ans += x;
                    i -= 2;
                }
            }
            s.resize(i, 0);
            ans
        }

        let (mut x, mut y) = (x, y);
        let mut s = s.as_bytes().to_vec();
        let mut ans = 0;
        let mut a = "ab".as_bytes();
        let mut b = "ba".as_bytes();
        if x < y {
            std::mem::swap(&mut a, &mut b);
            std::mem::swap(&mut x, &mut y);
        }
        ans += remove(&mut s, a, x);
        ans += remove(&mut s, b, y);
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let s = "cdbcbbaaabab".to_string();
        assert_eq!(Solution::maximum_gain(s, 4, 5), 19);
    }

    #[test]
    fn test_case_2() {
        let s = "aabbaaxybbaabb".to_string();
        assert_eq!(Solution::maximum_gain(s, 5, 4), 20);
    }
}
