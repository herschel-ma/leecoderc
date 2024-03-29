use std::collections::VecDeque;
use std::fmt::{Display, Formatter};
// Definition for a binary tree node.
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }

    pub fn get_height(root: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn dfs(root: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
            match root {
                None => 0,
                Some(node) => {
                    let node = node.borrow();
                    1 + dfs(&node.left).max(dfs(&node.right))
                }
            }
        }
        dfs(root)
    }

    /// helper: https://rsj217.github.io/leetcode-in-rust/
    pub fn from_vec(vals: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
        if vals.is_empty() {
            return None;
        }

        let size = vals.len();
        let root = Some(Rc::new(RefCell::new(Self::new(vals[0]))));
        let mut index = 0;
        let mut queue = VecDeque::new();
        queue.push_back(root.clone());

        while !queue.is_empty() {
            let q_size = queue.len();
            for _ in 0..q_size {
                if let Some(n) = queue.pop_front().flatten() {
                    let mut node = n.borrow_mut();
                    let lindex = index * 2 + 1;
                    let rindex = index * 2 + 2;
                    if lindex < size && vals[lindex] != i32::MIN {
                        node.left = Some(Rc::new(RefCell::new(Self::new(vals[lindex]))));
                        queue.push_back(node.left.clone());
                    }
                    if rindex < size && vals[rindex] != i32::MIN {
                        node.right = Some(Rc::new(RefCell::new(Self::new(vals[rindex]))));
                        queue.push_back(node.right.clone())
                    }
                }
                index += 1;
            }
        }

        root
    }

    fn _fill(&mut self, vals: &[i32], index: usize) {
        let left_node = index * 2 + 1;
        if left_node < vals.len() && vals[left_node] != i32::MIN {
            self.left = Some(Rc::new(RefCell::new(Self::new(vals[left_node]))));
            self.left
                .as_ref()
                .unwrap()
                .borrow_mut()
                ._fill(vals, left_node);
        }

        let right_node = left_node + 1;
        if right_node < vals.len() && vals[right_node] != i32::MIN {
            self.right = Some(Rc::new(RefCell::new(Self::new(vals[right_node]))));
            self.right
                .as_ref()
                .unwrap()
                .borrow_mut()
                ._fill(vals, right_node);
        }
    }

    pub fn into_array(&self) -> Vec<i32> {
        let mut result = Vec::new();
        self.walk(&mut result);
        result
    }

    fn walk(&self, result: &mut Vec<i32>) {
        if let Some(left) = &self.left {
            left.borrow().walk(result);
        }

        result.push(self.val);

        if let Some(right) = &self.right {
            right.borrow().walk(result);
        }
    }
}

impl Display for TreeNode {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        if self.val == i32::MIN {
            Ok(())
        } else {
            let left = if let Some(left) = &self.left {
                left.borrow().fmt(f)?;
                format!("{}", left.borrow().val)
            } else {
                "-".to_string()
            };

            let right = if let Some(right) = &self.right {
                right.borrow().fmt(f)?;
                format!("{}", right.borrow().val)
            } else {
                "-".to_string()
            };
            writeln!(f, "{}, left = {left}, right = {right}", self.val)
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn empty_tree() {
        assert!(TreeNode::from_vec(&[]).is_none());
    }

    #[test]
    fn fuller_tree() {
        let tree = TreeNode::from_vec(&[4, 2, 7, 1, 3, 6, 9]);
        let result = tree.unwrap().borrow().into_array();
        assert_eq!(result, vec![1, 2, 3, 4, 6, 7, 9]);
    }

    #[test]
    fn null_empty_tree() {
        let tree = TreeNode::from_vec(&[6, 2, 8, 0, 4, 7, 9, i32::MIN, i32::MIN, 3, 5]);
        let result = tree.unwrap().borrow().into_array();
        assert_eq!(result, vec![0, 2, 3, 4, 5, 6, 7, 8, 9]);
    }

    /*
                             6
                        2         8
                    0      4    7   9
                 N    N  3  5
            N 0 N 2 3 4 5 6 7 8 9
    */
}
