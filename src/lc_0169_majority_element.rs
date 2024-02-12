use crate::Solution;

impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        use std::collections::HashMap;
        let mut hm = HashMap::new();
        for n in nums {
            *hm.entry(n).or_insert(0) += 1
        }
        let mut count_vec = hm.into_iter().collect::<Vec<(i32, i32)>>();
        count_vec.sort_by(|a, b| b.1.cmp(&a.1));
        // println!("{:?}", count_vec);
        count_vec[0].0
    }

    pub fn majority_element_optimize(nums: Vec<i32>) -> i32 {
        let mut count = 1;
        let mut majaritoy = nums[0];

        for n in nums.iter().skip(1) {
            if majaritoy == *n {
                count += 1
            } else {
                count -= 1;
                if count == 0 {
                    majaritoy = *n;
                    count = 1;
                }
            }
        }

        majaritoy
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    type Func = fn(Vec<i32>) -> i32;
    const MA: Func = Solution::majority_element_optimize;

    #[test]
    fn ex1() {
        assert_eq!(MA(vec![3, 2, 3]), 3)
    }
    #[test]
    fn ex2() {
        assert_eq!(MA(vec![2, 2, 1, 1, 1, 2, 2]), 2)
    }
}
