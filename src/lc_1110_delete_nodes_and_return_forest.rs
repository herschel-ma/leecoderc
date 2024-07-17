use std::{cell::RefCell, rc::Rc};

use crate::{Solution, TreeNode};
use std::collections::VecDeque;

impl Solution {
    pub fn del_nodes(
        root: Option<Rc<RefCell<TreeNode>>>,
        to_delete: Vec<i32>,
    ) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        if root.is_none() {
            return vec![];
        }
        let mut res = Vec::new();
        let mut to_del_map = [0; 1001];
        for to_del in to_delete {
            to_del_map[to_del as usize] = 1;
        }
        let mut queue = VecDeque::new();
        queue.push_back((root.unwrap(), false));
        while let Some((root, prev)) = queue.pop_front() {
            let opt_left = root.borrow().left.clone();
            let opt_right = root.borrow().right.clone();
            let uval = root.borrow().val as usize;
            let next = to_del_map[uval] != 1;
            if !prev && next {
                res.push(Some(root.clone()));
            }
            if let Some(left) = opt_left {
                let check = to_del_map[left.borrow().val as usize] == 1;
                if check {
                    root.borrow_mut().left = None;
                }
                queue.push_back((left, next));
            }
            if let Some(right) = opt_right {
                let check = to_del_map[right.borrow().val as usize] == 1;
                if check {
                    root.borrow_mut().right = None;
                }
                queue.push_back((right, next));
            }
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let root = TreeNode::from_vec(&[1, 2, 3, 4, 5, 6, 7]);
        let to_delete = vec![3, 5];
        let output = vec![
            TreeNode::from_vec(&[1, 2, i32::MIN, 4]),
            TreeNode::from_vec(&[6]),
            TreeNode::from_vec(&[7]),
        ];
        assert_eq!(Solution::del_nodes(root, to_delete), output);
    }
    #[test]
    fn test_case_2() {
        let root = TreeNode::from_vec(&[1, 2, 4, i32::MIN, 3]);
        let to_delete = vec![3];
        let output = vec![TreeNode::from_vec(&[1, 2, 4])];
        assert_eq!(Solution::del_nodes(root, to_delete), output);
    }
}
