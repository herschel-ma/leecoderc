use crate::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
pub struct Solution;

impl Solution {
    pub fn count_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(r) = root {
            let lcount = {
                let mut count = 1;
                let mut node = r.borrow().left.clone();
                while let Some(n) = node {
                    count += 1;
                    node = n.borrow().left.clone();
                }
                count
            };
            let rcount = {
                let mut count = 1;
                let mut node = r.borrow().right.clone();
                while let Some(n) = node {
                    count += 1;
                    node = n.borrow().right.clone();
                }
                count
            };
            if lcount == rcount {
                2i32.pow(lcount) - 1
            } else {
                1 + Solution::count_nodes(r.borrow().left.clone())
                    + Solution::count_nodes(r.borrow().right.clone())
            }
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
        let root = vec![1, 2, 3, 4, 5, 6];
        let node = TreeNode::from_vec(&root);
        assert_eq!(Solution::count_nodes(node), 6);
    }
    #[test]
    fn ex2() {
        let node = None;
        assert_eq!(Solution::count_nodes(node), 0);
    }

    #[test]
    fn ex3() {
        let root = vec![1];
        let node = TreeNode::from_vec(&root);
        assert_eq!(Solution::count_nodes(node), 1);
    }
}
