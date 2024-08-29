use crate::{Solution, TreeNode};
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn postorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut ans: Vec<i32> = Vec::new();

        fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, ans: &mut Vec<i32>) {
            if let Some(n) = node {
                let n = n.borrow();

                dfs(&n.left, ans);
                dfs(&n.right, ans);

                ans.push(n.val);
            }
        }

        dfs(&root, &mut ans);
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let root = TreeNode::from_vec(&[1, i32::MIN, 2, 3]);
        dbg!(&root);
        assert_eq!(Solution::postorder_traversal(root), vec![3, 2, 1]);
    }

    #[test]
    fn test_case_2() {
        let root = TreeNode::from_vec(&[]);
        let res: Vec<i32> = Vec::new();
        assert_eq!(Solution::postorder_traversal(root), res);
    }

    #[test]
    fn test_case_3() {
        let root = TreeNode::from_vec(&[1]);
        assert_eq!(Solution::postorder_traversal(root), vec![1]);
    }

    #[test]
    fn test_case_4() {
        let root = TreeNode::from_vec(&[1, 2, 3, 4, 5]);
        assert_eq!(Solution::postorder_traversal(root), vec![4, 5, 2, 3, 1]);
    }
}
