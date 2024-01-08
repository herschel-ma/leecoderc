use std::{cell::RefCell, rc::Rc};

use crate::{Solution, TreeNode};

impl Solution {
    pub fn range_sum_bst(root: Option<Rc<RefCell<TreeNode>>>, low: i32, high: i32) -> i32 {
        if let Some(node) = root {
            let n = node.borrow();
            if n.val >= low && n.val <= high {
                return n.val
                    + Self::range_sum_bst(n.left.clone(), low, high)
                    + Self::range_sum_bst(n.right.clone(), low, high);
            } else if n.val < low {
                return Self::range_sum_bst(n.right.clone(), low, high);
            } else if n.val > high {
                return Self::range_sum_bst(n.left.clone(), low, high);
            };
        }

        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let root = TreeNode::from_vec(&[10, 5, 15, 3, 7, i32::MIN, 18]);
        let low = 7;
        let high = 15;
        assert_eq!(32, Solution::range_sum_bst(root, low, high));
    }

    #[test]
    fn test_case_2() {
        let root = TreeNode::from_vec(&[10, 5, 15, 3, 7, 13, 18, 1, i32::MIN, 6]);
        let low = 6;
        let high = 10;
        assert_eq!(23, Solution::range_sum_bst(root, low, high));
    }
}

