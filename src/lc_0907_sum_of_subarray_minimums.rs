use crate::Solution;

impl Solution {
    pub fn sum_subarray_mins(mut arr: Vec<i32>) -> i32 {
        arr.insert(0, i32::MIN);
        arr.push(i32::MIN);

        let mut res = 0;
        let mut stack = Vec::new();

        for i in 0..arr.len() {
            while !stack.is_empty() && arr[*stack.last().unwrap()] > arr[i] {
                let curr = stack.pop().unwrap();
                res += arr[curr] as usize * (i - curr) * (curr - *stack.last().unwrap())
            }
            stack.push(i)
        }

        (res % (10usize.pow(9) + 7)) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let arr = vec![3, 1, 2, 4];
        assert_eq!(Solution::sum_subarray_mins(arr), 17);
    }

    #[test]
    fn test_case_2() {
        let arr = vec![11, 81, 94, 43, 3];
        assert_eq!(Solution::sum_subarray_mins(arr), 444);
    }
}

