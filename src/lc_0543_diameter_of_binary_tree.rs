use crate::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;
impl Solution {
    pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::diameter(&root)
    }

    fn diameter(root: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(node) = root {
            let left_height = Self::height(&node.borrow().left);
            let right_height = Self::height(&node.borrow().right);

            let left_diameter = Self::diameter(&node.borrow().left);
            let right_diameter = Self::diameter(&node.borrow().right);
            (left_height + right_height).max(left_diameter.max(right_diameter))
        } else {
            0
        }
    }
    fn height(root: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(node) = root {
            let left = &node.borrow().left;
            let right = &node.borrow().right;
            Self::height(left).max(Self::height(right)) + 1
        } else {
            0
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let tree = TreeNode::from_vec(&vec![1, 2, 3, 4, 5]);
        assert_eq!(Solution::diameter_of_binary_tree(tree), 3);
    }

    #[test]
    fn ex2() {
        let tree = TreeNode::from_vec(&vec![1, 2]);
        assert_eq!(Solution::diameter_of_binary_tree(tree), 1);
    }
    #[test]
    fn wrong_answer() {
        let tree = TreeNode::from_vec(&vec![
            4,
            -7,
            -3,
            i32::MIN,
            i32::MIN,
            -9,
            -3,
            9,
            -7,
            -4,
            i32::MIN,
            6,
            i32::MIN,
            -6,
            -6,
            i32::MIN,
            i32::MIN,
            0,
            6,
            5,
            i32::MIN,
            9,
            i32::MIN,
            i32::MIN,
            -1,
            -4,
            i32::MIN,
            i32::MIN,
            i32::MIN,
            -2,
        ]);
        assert_eq!(Solution::diameter_of_binary_tree(tree), 8)
    }
}