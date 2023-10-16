use crate::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    diameter(&root)
}

pub fn diameter(root: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
    if let Some(node) = root {
        let left_height = height(&node.borrow().left);
        let right_height = height(&node.borrow().right);

        let left_diameter = diameter(&node.borrow().left);
        let right_diameter = diameter(&node.borrow().right);
        (left_height + right_height).max(left_diameter.max(right_diameter))
    } else {
        0
    }
}

pub fn height(root: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
    if let Some(node) = root {
        let left = &node.borrow().left;
        let right = &node.borrow().right;
        height(left).max(height(right)) + 1
    } else {
        0
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let tree = TreeNode::from_vec(&vec![1, 2, 3, 4, 5]);
        assert_eq!(diameter_of_binary_tree(tree), 3);
    }

    #[test]
    fn ex2() {
        let tree = TreeNode::from_vec(&vec![1, 2]);
        assert_eq!(diameter_of_binary_tree(tree), 1);
    }
}
