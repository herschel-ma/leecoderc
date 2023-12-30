use crate::Solution;
use crate::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn is_same_tree(
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        match (p, q) {
            (None, None) => true,
            (Some(p), Some(q)) => {
                let p = p.borrow();
                let q = q.borrow();
                p.val == q.val
                    && Self::is_same_tree(p.left.clone(), q.left.clone())
                    && Self::is_same_tree(p.right.clone(), q.right.clone())
            }
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let p = TreeNode::from_vec(&[1, 2, 3]);
        let q = TreeNode::from_vec(&[1, 2, 3]);
        assert!(Solution::is_same_tree(p, q));
    }

    #[test]
    fn test_case_2() {
        let p = TreeNode::from_vec(&[1, 2]);
        let q = TreeNode::from_vec(&[1, i32::MIN, 2]);
        assert!(!Solution::is_same_tree(p, q));
    }

    #[test]
    fn test_case_3() {
        let p = TreeNode::from_vec(&[1, 2, 1]);
        let q = TreeNode::from_vec(&[1, 1, 2]);
        assert!(!Solution::is_same_tree(p, q));
    }
}

