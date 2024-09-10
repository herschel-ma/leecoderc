use crate::{ListNode, Solution};
impl Solution {
    pub fn spiral_matrix(m: i32, n: i32, head: Option<Box<ListNode>>) -> Vec<Vec<i32>> {
        let mut head = head;
        let mut res = vec![vec![-1; n as usize]; m as usize];
        let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
        let mut mov = 0;
        let (mut i, mut j) = (0, 0);
        while let Some(node) = head {
            res[i as usize][j as usize] = node.val;
            head = node.next;
            let nexti = (i + directions[mov].0 + m) % m;
            let nextj = (j + directions[mov].1 + n) % n;
            if res[nexti as usize][nextj as usize] != -1 {
                mov = (mov + 1) % 4;
            }
            i += directions[mov].0;
            j += directions[mov].1;
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let head = ListNode::from_vec(&[3, 0, 2, 6, 8, 1, 7, 9, 4, 2, 5, 5, 0]);
        let res = vec![
            vec![3, 0, 2, 6, 8],
            vec![5, 0, -1, -1, 1],
            vec![5, 2, 4, 9, 7],
        ];
        assert_eq!(Solution::spiral_matrix(3, 5, head), res);
    }

    #[test]
    fn test_case_2() {
        let head = ListNode::from_vec(&[0, 1, 2]);
        let res = vec![vec![0, 1, 2, -1]];
        assert_eq!(Solution::spiral_matrix(1, 4, head), res);
    }
}
