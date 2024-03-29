use std::collections::HashMap;

#[allow(dead_code)]
pub fn two_sum_old(nums: Vec<i32>, target: i32) -> Vec<i32> {
    for i in 0..nums.len() - 1 {
        for j in i + 1..nums.len() {
            if nums[i] + nums[j] == target {
                return vec![i as i32, j as i32];
            }
        }
    }
    Vec::new()
}

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut idx: HashMap<i32, usize> = HashMap::new();
    for (i, num) in nums.iter().enumerate() {
        let diff = target - num;
        if let Some(&j) = idx.get(&diff) {
            return vec![j as i32, i as i32];
        }
        idx.insert(*num, i);
    }
    Vec::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let list = vec![2, 7, 11, 15];
        let result = vec![0, 1];
        assert_eq!(two_sum(list, 9), result);
    }
    #[test]
    fn ex2() {
        let list = vec![3, 2, 4];
        let result = vec![1, 2];
        assert_eq!(two_sum(list, 6), result);
    }
    #[test]
    fn ex3() {
        let list = vec![3, 3];
        let result = vec![0, 1];
        assert_eq!(two_sum(list, 6), result);
    }
}
