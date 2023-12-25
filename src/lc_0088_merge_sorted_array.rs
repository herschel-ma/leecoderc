use crate::Solution;

impl Solution {
    pub fn merge_(nums1: &mut [i32], m: i32, nums2: &mut [i32], n: i32) {
        let (mut m, mut n) = (m as usize, n as usize);
        for i in (0..m + n).rev() {
            nums1[i] = match (m == 0, n == 0) {
                (false, true) => {
                    m -= 1;
                    nums1[m]
                }
                (true, false) => {
                    n -= 1;
                    nums2[n]
                }
                (_, _) => {
                    if nums1[m - 1] > nums2[n - 1] {
                        m -= 1;
                        nums1[m]
                    } else {
                        n -= 1;
                        nums2[n]
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let mut nums1 = vec![1, 2, 3, 0, 0, 0];
        let mut nums2 = vec![2, 5, 6];
        let (m, n) = (3, 3);
        Solution::merge_(&mut nums1, m, &mut nums2, n);
        assert_eq!(nums1, vec![1, 2, 2, 3, 5, 6]);
    }

    #[test]
    fn test_case_2() {
        let mut nums1 = vec![1];
        let mut nums2 = vec![];
        let (m, n) = (1, 0);
        Solution::merge_(&mut nums1, m, &mut nums2, n);
        assert_eq!(nums1, vec![1]);
    }

    #[test]
    fn test_case_3() {
        let mut nums1 = vec![0];
        let mut nums2 = vec![1];
        let (m, n) = (0, 1);
        Solution::merge_(&mut nums1, m, &mut nums2, n);
        assert_eq!(nums1, vec![1]);
    }
}
