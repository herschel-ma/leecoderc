pub fn search(nums: Vec<i32>, target: i32) -> i32 {
    // nums.binary_search(&target).unwrap().try_into().unwrap()
    if nums.is_empty() {
        return -1;
    }
    let mut low = 0;
    let mut heigh = nums.len() - 1;
    let mut mid = (low + heigh) >> 1;
    while nums[mid] != target {
        if low >= heigh {
            return -1;
        }

        if nums[mid] < target {
            low = mid + 1;
        } else {
            heigh = mid.saturating_sub(1);
        }

        mid = (low + heigh) >> 1;
    }
    mid as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(search(vec![-1, 0, 3, 5, 9, 12], 9), 4);
    }

    #[test]
    fn ex2() {
        assert_eq!(search(vec![-1, 0, 3, 5, 9, 12], 2), -1);
    }

    #[test]
    fn ex3() {
        assert_eq!(search(vec![], 3), -1);
    }

    #[test]
    fn ex4() {
        assert_eq!(search(vec![2, 5], 0), -1);
    }
}
