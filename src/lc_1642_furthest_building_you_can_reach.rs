use crate::Solution;

use std::{cmp::Reverse, collections::BinaryHeap};

impl Solution {
    pub fn furthest_building(heights: Vec<i32>, mut bricks: i32, ladders: i32) -> i32 {
        let mut min_heap = BinaryHeap::new();
        (1..heights.len())
            .map(|i| heights[i] - heights[i - 1])
            .take_while(|&diff| {
                if diff > 0 {
                    min_heap.push(Reverse(diff));
                    if min_heap.len() > ladders as usize {
                        bricks -= min_heap.pop().unwrap().0;
                    }
                }
                bricks >= 0
            })
            .count() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let heights = vec![4, 2, 7, 6, 9, 14, 12];
        let bricks = 5;
        let ladders = 1;
        assert_eq!(Solution::furthest_building(heights, bricks, ladders), 4);
    }

    #[test]
    fn test_case_2() {
        let heights = vec![4, 12, 2, 7, 3, 18, 20, 3, 19];
        let bricks = 10;
        let ladders = 2;
        assert_eq!(Solution::furthest_building(heights, bricks, ladders), 7);
    }

    #[test]
    fn test_case_3() {
        let heights = vec![14, 3, 19, 3];
        let bricks = 17;
        let ladders = 0;
        assert_eq!(Solution::furthest_building(heights, bricks, ladders), 3);
    }
}
