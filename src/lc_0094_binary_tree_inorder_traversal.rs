use std::{cell::RefCell, rc::Rc};

use crate::{Solution, TreeNode};

impl Solution {
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut res = vec![];
        fn helper(root: &Option<Rc<RefCell<TreeNode>>>, res: &mut Vec<i32>) {
            if root.is_none() {
                return;
            }
            let node = root.as_ref().unwrap().borrow();
            helper(&node.left, res);
            res.push(node.val);
            helper(&node.right, res);
        }

        helper(&root, &mut res);
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let root = TreeNode::from_vec(&[1, i32::MIN, 2, 3]);
        assert_eq!(Solution::inorder_traversal(root), vec![1, 3, 2]);
    }

    #[test]
    fn test_case_2() {
        let root = TreeNode::from_vec(&[]);
        assert_eq!(Solution::inorder_traversal(root), vec![]);
    }

    #[test]
    fn test_case_3() {
        let root = TreeNode::from_vec(&[1]);
        assert_eq!(Solution::inorder_traversal(root), vec![1]);
    }
}
