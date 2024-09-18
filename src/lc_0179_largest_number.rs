use crate::Solution;
impl Solution {
    pub fn largest_number(nums: Vec<i32>) -> String {
        let mut nums = nums
            .into_iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>();
        nums.sort_by(|a, b| {
            let a = format!("{}{}", a, b);
            let b = format!("{}{}", b, a);
            b.cmp(&a)
        });
        if let Some(num) = nums.first() {
            if num == "0" {
                return num.to_string();
            }
        }
        nums.join("")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let nums = vec![10, 2];
        let output = "210".to_string();
        assert_eq!(Solution::largest_number(nums), output);
    }

    #[test]
    fn test_case_2() {
        let nums = vec![3, 30, 34, 5, 9];
        let output = "9534330".to_string();
        assert_eq!(Solution::largest_number(nums), output);
    }

    #[test]
    fn test_case_3() {
        let nums = vec![0, 0];
        assert_eq!(Solution::largest_number(nums), "0".to_string());
    }
}
