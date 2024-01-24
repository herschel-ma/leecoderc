use std::{cell::RefCell, collections::HashSet, rc::Rc};

use crate::{Solution, TreeNode};

impl Solution {
    pub fn pseudo_palindromic_paths(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn dfs(node: Option<&Rc<RefCell<TreeNode>>>, mut pa: HashSet<i32>) -> i32 {
            if let Some(node_ref) = node {
                let node = node_ref.borrow();
                if pa.contains(&node.val) {
                    pa.remove(&node.val);
                } else {
                    pa.insert(node.val);
                }

                if node.left.is_none() && node.right.is_none() && pa.len() <= 1 {
                    return 1;
                }

                let left = dfs(node.left.as_ref(), pa.clone());
                let right = dfs(node.right.as_ref(), pa.clone());

                return left + right;
            }

            0
        }

        dfs(root.as_ref(), HashSet::new())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let root = TreeNode::from_vec(&[2, 3, 1, 3, 1, i32::MIN, 1]);
        assert_eq!(2, Solution::pseudo_palindromic_paths(root));
    }

    #[test]
    fn test_case_2() {
        let toot = TreeNode::from_vec(&[
            2,
            1,
            1,
            1,
            3,
            i32::MIN,
            i32::MIN,
            i32::MIN,
            i32::MIN,
            i32::MIN,
            1,
        ]);
        assert_eq!(1, Solution::pseudo_palindromic_paths(toot));
    }
    #[test]
    fn test_case_3() {
        let toot = TreeNode::from_vec(&[
            8,
            8,
            i32::MIN,
            7,
            7,
            i32::MIN,
            i32::MIN,
            2,
            4,
            i32::MIN,
            8,
            i32::MIN,
            7,
            i32::MIN,
            1,
        ]);
        assert_eq!(2, Solution::pseudo_palindromic_paths(toot));
    }
}
