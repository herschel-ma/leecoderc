use std::{cell::RefCell, rc::Rc};

use crate::{Solution, TreeNode};

impl Solution {
    pub fn max_ancestor_diff(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn helper(root: &Option<Rc<RefCell<TreeNode>>>, cur_max: i32, cur_min: i32) -> i32 {
            match root {
                Some(node) => {
                    let node = node.borrow();
                    let val = node.val;

                    let low = cur_min.min(val);
                    let high = cur_max.max(val);

                    ((low - val).abs())
                        .max((high - val).abs())
                        .max(helper(&node.left, high, low))
                        .max(helper(&node.right, high, low))
                }
                None => 0,
            }
        }

        helper(&root, i32::MIN, i32::MAX)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let root =
            TreeNode::from_vec(&[8, 3, 10, 1, 6, i32::MIN, 14, i32::MIN, i32::MIN, 4, 7, 13]);
        assert_eq!(Solution::max_ancestor_diff(root), 7);
    }

    #[test]
    fn test_case_2() {
        let root = TreeNode::from_vec(&[1, i32::MIN, 2, i32::MIN, 0, 3]);
        assert_eq!(Solution::max_ancestor_diff(root), 3);
    }
}

