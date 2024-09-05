use crate::Solution;

impl Solution {
    pub fn missing_rolls(rolls: Vec<i32>, mean: i32, n: i32) -> Vec<i32> {
        let m = rolls.len();
        let mut ans = Vec::new();
        let remain = mean * (m as i32 + n) - rolls.iter().sum::<i32>();

        if n > remain || n * 6 < remain {
            return ans;
        }
        ans = vec![remain / n; n as usize];
        ans[..(remain % n) as usize]
            .iter_mut()
            .for_each(|v| *v += 1);

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let mut e = Solution::missing_rolls(vec![3, 2, 4, 3], 4, 2);
        let mut v = vec![6, 6];
        e.sort();
        v.sort();
        assert_eq!(e, v);
    }

    #[test]
    fn test_case_2() {
        let mut e = Solution::missing_rolls(vec![1, 5, 6], 3, 4);
        let mut v = vec![2, 3, 2, 2];
        v.sort();
        e.sort();
        assert_eq!(e, v);
    }

    #[test]
    fn test_case_3() {
        let mut v: Vec<i32> = Vec::new();
        let mut e = Solution::missing_rolls(vec![1, 2, 3, 4], 6, 4);
        e.sort();
        v.sort();
        assert_eq!(e, v);
    }
}
