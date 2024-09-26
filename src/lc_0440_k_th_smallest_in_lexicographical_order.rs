use crate::Solution;

impl Solution {
    pub fn find_kth_number(n: i32, k: i32) -> i32 {
        fn count(mut curr: i64, n: i64) -> i64 {
            let mut next = curr + 1;
            let mut cnt = 0;
            while curr <= n {
                cnt += std::cmp::min(n.saturating_sub(curr) + 1, next - curr);
                curr *= 10;
                next *= 10;
            }

            cnt
        }

        let mut curr = 1_i64;
        let mut k = k as i64 - 1;

        while k > 0 {
            let cnt = count(curr, n as i64);
            if k >= cnt {
                k -= cnt;
                curr += 1;
            } else {
                k -= 1;
                curr *= 10;
            }
        }

        curr as _
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(Solution::find_kth_number(13, 2), 10);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(Solution::find_kth_number(1, 1), 1);
    }

    #[test]
    fn test_case_3() {
        assert_eq!(Solution::find_kth_number(2, 2), 2);
    }
}
