use crate::{Solution, TreeNode};
use std::{cell::RefCell, rc::Rc};

impl Solution {
    pub fn leaf_similar(
        root1: Option<Rc<RefCell<TreeNode>>>,
        root2: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        fn helper(node: &Option<Rc<RefCell<TreeNode>>>, v: &mut Vec<i32>) {
            if let Some(node) = node {
                let node = node.borrow();
                if node.left.is_none() && node.right.is_none() {
                    v.push(node.val);
                }
                helper(&node.left, v);
                helper(&node.right, v);
            }
        }

        let mut v1 = Vec::new();
        let mut v2 = Vec::new();
        helper(&root1, &mut v1);
        helper(&root2, &mut v2);

        v1 == v2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let root1 = TreeNode::from_vec(&[3, 5, 1, 6, 2, 9, 8, i32::MIN, i32::MIN, 7, 4]);
        let root2 = TreeNode::from_vec(&[
            3,
            5,
            1,
            6,
            7,
            4,
            2,
            i32::MIN,
            i32::MIN,
            i32::MIN,
            i32::MIN,
            i32::MIN,
            i32::MIN,
            9,
            8,
        ]);
        assert!(Solution::leaf_similar(root1, root2));
    }

    #[test]
    fn test_case_2() {
        let root1 = TreeNode::from_vec(&[1, 2, 3]);
        let root2 = TreeNode::from_vec(&[1, 3, 2]);
        assert!(!Solution::leaf_similar(root1, root2));
    }
}

