use crate::Solution;

impl Solution {
    pub fn largest_rectangle_area_1(heights: Vec<i32>) -> i32 {
        let n = heights.len();
        let mut left_boundaries = vec![0; n];
        let mut right_boundaries = vec![0; n];
        let mut stack = Vec::new();

        // step.1 Find left boundary
        for i in 0..n {
            while let Some(&top) = stack.last() {
                if heights[top] < heights[i] {
                    break;
                }
                stack.pop();
            }
            left_boundaries[i] = if stack.is_empty() {
                -1
            } else {
                stack.last().copied().unwrap() as i32
            };
            stack.push(i);
        }
        stack.clear();

        // step.2 Find right boundary
        for i in (0..n).rev() {
            while let Some(&top) = stack.last() {
                if heights[top] < heights[i] {
                    break;
                }
                stack.pop();
            }
            right_boundaries[i] = if stack.is_empty() {
                n as i32
            } else {
                stack.last().copied().unwrap() as i32
            };
            stack.push(i);
        }
        // step.3 Calculate area
        let mut max_area = 0;
        for i in 0..n {
            let width = right_boundaries[i] - left_boundaries[i] - 1;
            let area = heights[i] * width;
            max_area = max_area.max(area);
        }
        max_area
    }

    pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
        let n = heights.len();
        let mut max_area = 0;
        let mut stack: Vec<(usize, i32)> = Vec::new();
        for (i, h) in heights.into_iter().enumerate() {
            let mut start = i; // If it can continuous expand forward.
            while !stack.is_empty() && stack.last().unwrap().1 >= h {
                let (index, height) = stack.pop().unwrap();
                max_area = max_area.max(height * (i - index) as i32);
                start = index;
            }
            stack.push((start, h));
        }

        for (_, h) in stack.into_iter().enumerate() {
            max_area = max_area.max(h.1 * (n - h.0) as i32);
        }

        max_area
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let heights = vec![2, 1, 5, 6, 2, 3];
        assert_eq!(10, Solution::largest_rectangle_area(heights));
    }

    #[test]
    fn test_case_2() {
        let heights = vec![2, 4];
        assert_eq!(4, Solution::largest_rectangle_area(heights));
    }
}

