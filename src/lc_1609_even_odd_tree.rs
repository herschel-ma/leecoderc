use crate::{Solution, TreeNode};
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

impl Solution {
    pub fn is_even_odd_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if root.is_none() {
            return false;
        }

        let mut level = 0;
        let (mut queue, mut temp) = (VecDeque::new(), VecDeque::new());
        queue.push_back(root.unwrap());

        while !queue.is_empty() {
            let mut pre = if level % 2 == 0 { i32::MIN } else { i32::MAX };
            while let Some(n) = queue.pop_front() {
                let node = n.borrow();
                if (level % 2 == 0 && (node.val % 2 == 0 || node.val <= pre))
                    || (level % 2 == 1 && (node.val % 2 == 1 || node.val >= pre))
                {
                    return false;
                }
                pre = node.val;

                if let Some(l) = node.left.clone() {
                    temp.push_back(l)
                }
                if let Some(r) = node.right.clone() {
                    temp.push_back(r)
                }
            }
            std::mem::swap(&mut temp, &mut queue);
            level += 1;
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let root =
            TreeNode::from_vec(&[1, 10, 4, 3, i32::MIN, 7, 9, 12, 8, 6, i32::MIN, i32::MIN, 2]);
        assert!(Solution::is_even_odd_tree(root));
    }
    #[test]
    fn test_case_2() {
        let root = TreeNode::from_vec(&[5, 4, 2, 3, 3, 7]);
        assert!(!Solution::is_even_odd_tree(root));
    }
    #[test]
    fn test_case_3() {
        let root = TreeNode::from_vec(&[5, 9, 1, 3, 5, 7]);
        assert!(!Solution::is_even_odd_tree(root));
    }
}
