use crate::Solution;
use crate::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

type OptNode = Option<Rc<RefCell<TreeNode>>>;

impl Solution {
    pub fn is_valid_bst(root: OptNode) -> bool {
        fn helper(node: &OptNode, min: i64, max: i64) -> bool {
            match node.as_ref() {
                None => true,
                Some(n) => {
                    let b = n.borrow();
                    (b.val as i64) > min
                        && (b.val as i64) < max
                        && helper(&b.left, min, b.val as i64)
                        && helper(&b.right, b.val as i64, max)
                }
            }
        }

        helper(&root, i32::MIN as i64 - 1, i32::MAX as i64 + 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let root = TreeNode::from_vec(&[2, 1, 3]);
        assert!(Solution::is_valid_bst(root));
    }

    #[test]
    fn test_case_2() {
        let root = TreeNode::from_vec(&[5, 1, 4, i32::MIN, i32::MIN, 3, 6]);
        assert!(!Solution::is_valid_bst(root));
    }
}

