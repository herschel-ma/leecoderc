use std::{cell::RefCell, rc::Rc};

use crate::{Solution, TreeNode};

impl Solution {
    pub fn find_bottom_left_value(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut level = Vec::from([root.unwrap()]);
        let mut ans: i32 = 0;
        while !level.is_empty() {
            ans = level[0].borrow().val;
            level = level
                .into_iter()
                .flat_map(|n| [n.borrow().left.clone(), n.borrow().right.clone()])
                .flatten()
                .collect();
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use crate::TreeNode;

    use super::*;

    #[test]
    fn test_case_1() {
        let root = TreeNode::from_vec(&[2, 1, 3]);
        assert_eq!(Solution::find_bottom_left_value(root), 1);
    }

    #[test]
    fn test_case_2() {
        let root = TreeNode::from_vec(&[1, 2, 3, 4, i32::MIN, 5, 6, i32::MIN, i32::MIN, 7]);
        assert_eq!(Solution::find_bottom_left_value(root), 7);
    }
}
