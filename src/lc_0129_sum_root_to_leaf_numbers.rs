use std::{cell::RefCell, rc::Rc};

use crate::{Solution, TreeNode};

impl Solution {
    pub fn sum_numbers(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut sum = 0;
        fn helper(node: &Option<Rc<RefCell<TreeNode>>>, sum: &mut i32, cur_sum: i32) {
            if let Some(n) = node {
                let n = n.borrow();
                let cur_sum = cur_sum * 10 + n.val;

                if n.left.is_none() && n.right.is_none() {
                    *sum += cur_sum;
                }

                helper(&n.left, sum, cur_sum);
                helper(&n.right, sum, cur_sum);
            }
        }

        helper(&root, &mut sum, 0);

        sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let root = TreeNode::from_vec(&[1, 2, 3]);
        assert_eq!(Solution::sum_numbers(root), 25);
    }

    #[test]
    fn test_case_2() {
        let root = TreeNode::from_vec(&[4, 9, 0, 5, 1]);
        assert_eq!(Solution::sum_numbers(root), 1026);
    }
}

