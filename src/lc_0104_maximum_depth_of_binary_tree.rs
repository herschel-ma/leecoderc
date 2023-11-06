use crate::TreeNode;
use std::{cell::RefCell, rc::Rc};

pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    depth(&root)
}
fn depth(root: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
    if let Some(node) = root {
        let left_depth = depth(&node.as_ref().borrow().left);
        let right_depth = depth(&node.as_ref().borrow().right);
        left_depth.max(right_depth) + 1
    } else {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn ex1() {
        let root = TreeNode::from_vec(&[3, 9, 20, i32::MIN, i32::MIN, 15, 7]);
        assert_eq!(max_depth(root), 3);
    }

    #[test]
    fn ex2() {
        let root = TreeNode::from_vec(&[1, i32::MIN, 2]);
        assert_eq!(max_depth(root), 2)
    }
}
