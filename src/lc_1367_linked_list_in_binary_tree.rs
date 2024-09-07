use crate::{ListNode, Solution, TreeNode};
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn is_sub_path(head: Option<Box<ListNode>>, root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        fn dfs(head: &Option<Box<ListNode>>, root: &Option<Rc<RefCell<TreeNode>>>) -> bool {
            if head.is_none() {
                return true;
            }
            if root.is_none() {
                return false;
            }
            let head = head.as_ref().unwrap();
            let root = root.as_ref().unwrap().borrow();
            if head.val == root.val {
                return dfs(&head.next, &root.left) || dfs(&head.next, &root.right);
            }
            false
        }

        if root.is_none() {
            return false;
        }

        fn my_is_sub_path(
            head: &Option<Box<ListNode>>,
            root: &Option<Rc<RefCell<TreeNode>>>,
        ) -> bool {
            if root.is_none() {
                return false;
            }

            dfs(head, root)
                || my_is_sub_path(head, &root.as_ref().unwrap().borrow().left)
                || my_is_sub_path(head, &root.as_ref().unwrap().borrow().right)
        }

        my_is_sub_path(&head, &root)
    }

    pub fn is_sub_path_2(head: Option<Box<ListNode>>, root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        fn get_list(mut head: Option<Box<ListNode>>) -> Vec<i32> {
            let mut res = vec![];
            while let Some(node) = head {
                res.push(node.val);
                head = node.next;
            }
            res
        }
        let nodes = get_list(head);
        let mut stack = vec![(root.unwrap(), vec![])];
        while let Some((node, mut path)) = stack.pop() {
            path.push(node.as_ref().borrow().val);
            if path.len() > nodes.len() || nodes[0..path.len()] != path {
                path.remove(0);
            }
            if nodes == path {
                return true;
            }
            if node.as_ref().borrow().left.is_some() {
                stack.push((node.as_ref().borrow().left.clone().unwrap(), path.clone()));
            }
            if node.as_ref().borrow().right.is_some() {
                stack.push((node.as_ref().borrow().right.clone().unwrap(), path.clone()));
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    type So = fn(head: Option<Box<ListNode>>, root: Option<Rc<RefCell<TreeNode>>>) -> bool;
    const S1: So = Solution::is_sub_path;
    const S2: So = Solution::is_sub_path_2;

    #[test]
    fn test_case_1() {
        let head = ListNode::from_vec(&[4, 2, 8]);
        let root = TreeNode::from_vec(&[
            1,
            4,
            4,
            i32::MIN,
            2,
            2,
            i32::MIN,
            1,
            i32::MIN,
            6,
            8,
            i32::MIN,
            i32::MIN,
            i32::MIN,
            i32::MIN,
            1,
            3,
        ]);
        assert!(S1(head.clone(), root.clone()));
        assert!(S2(head, root));
    }

    #[test]
    fn test_case_2() {
        let head = ListNode::from_vec(&[1, 4, 2, 6]);
        let root = TreeNode::from_vec(&[
            1,
            4,
            4,
            i32::MIN,
            2,
            2,
            i32::MIN,
            1,
            i32::MIN,
            6,
            8,
            i32::MIN,
            i32::MIN,
            i32::MIN,
            i32::MIN,
            1,
            3,
        ]);
        assert!(S1(head.clone(), root.clone()));
        assert!(S2(head, root));
    }

    #[test]
    fn test_case_3() {
        let head = ListNode::from_vec(&[1, 4, 2, 6, 8]);
        let root = TreeNode::from_vec(&[
            1,
            4,
            4,
            i32::MIN,
            2,
            2,
            i32::MIN,
            1,
            i32::MIN,
            6,
            8,
            i32::MIN,
            i32::MIN,
            i32::MIN,
            i32::MIN,
            1,
            3,
        ]);
        assert!(!S1(head.clone(), root.clone()));
        assert!(!S2(head, root));
    }
}
