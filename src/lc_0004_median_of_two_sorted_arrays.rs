pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    let sz1 = nums1.len() as i32;
    let sz2 = nums2.len() as i32;
    if sz1 > sz2 {
        return find_median_sorted_arrays(nums2, nums1);
    }
    let sz = sz1 + sz2;
    let half = (sz + 1) / 2;
    let mut low = 0;
    let mut high = sz1;
    while low <= high {
        let mid1 = (low + high) / 2;
        let mid2 = half - mid1;
        let r1 = if mid1 < sz1 {
            nums1[mid1 as usize]
        } else {
            i32::MAX
        };
        let r2 = if mid2 < sz2 {
            nums2[mid2 as usize]
        } else {
            i32::MAX
        };
        let l1 = if mid1 > 0 {
            nums1[mid1 as usize - 1]
        } else {
            i32::MIN
        };
        let l2 = if mid2 > 0 {
            nums2[mid2 as usize - 1]
        } else {
            i32::MIN
        };
        if l1 <= r2 && l2 <= r1 {
            if sz % 2 == 1 {
                return l1.max(l2) as f64;
            }
            return (l1.max(l2) + r1.min(r2)) as f64 / 2.0;
        } else if l1 > r2 {
            high = mid1 - 1;
        } else {
            low = mid1 + 1;
        }
    }
    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case1() {
        let nums1 = vec![1, 3];
        let nums2 = vec![2];
        let output = 2.000_000;
        assert_eq!(find_median_sorted_arrays(nums1, nums2), output);
    }

    #[test]
    fn test_case2() {
        let nums1 = vec![1, 2];
        let nums2 = vec![3, 4];
        let output = 2.500_00;
        assert_eq!(find_median_sorted_arrays(nums1, nums2), output);
    }
}
