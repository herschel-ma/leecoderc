use crate::Solution;

impl Solution {
    pub fn maximal_rectangle(matrix: Vec<Vec<char>>) -> i32 {
        let mut max_area = 0;
        let mut heights = vec![0; matrix[0].len()];
        for (_i, row) in matrix.iter().enumerate() {
            for (j, v) in row.iter().enumerate() {
                if v.to_digit(2) == Some(1) {
                    heights[j] += 1;
                } else {
                    heights[j] = 0;
                }
            }

            max_area = max_area.max(largest_rectangle_area(&heights));
        }

        fn largest_rectangle_area(heights: &Vec<i32>) -> i32 {
            let mut stack: Vec<(usize, i32)> = vec![];
            let mut max_area = 0;
            for (i, h) in heights.iter().enumerate() {
                let mut start = i;
                while !stack.is_empty() && stack.last().unwrap().1 > *h {
                    let (index, height) = stack.pop().unwrap();
                    max_area = max_area.max(height * (i - index) as i32);
                    start = index;
                }
                stack.push((start, *h));
            }

            for (i, h) in stack.iter() {
                max_area = max_area.max(h * (heights.len() - i) as i32)
            }
            max_area
        }

        max_area
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let matrix = vec![
            vec!['1', '0', '1', '0', '0'],
            vec!['1', '0', '1', '1', '1'],
            vec!['1', '1', '1', '1', '1'],
            vec!['1', '0', '0', '1', '0'],
        ];
        assert_eq!(Solution::maximal_rectangle(matrix), 6);
    }

    #[test]
    fn test_case_2() {
        let matrix = vec![vec!['0']];
        assert_eq!(Solution::maximal_rectangle(matrix), 0);
    }

    #[test]
    fn test_case_3() {
        let matrix = vec![vec!['1']];
        assert_eq!(Solution::maximal_rectangle(matrix), 1);
    }
}

