use crate::tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

fn r(root: &Option<Rc<RefCell<TreeNode>>>, res: &mut i32, count: &mut i32, s: &mut i32) {
    if let Some(root) = root {
        let node = root.borrow();
        let mut t = node.val;
        let mut m = 1;

        r(&node.left, res, &mut m, &mut t);
        r(&node.right, res, &mut m, &mut t);

        if node.val == t / m {
            *res += 1
        }

        *s += t;
        *count += m;
    }
}

pub fn average_of_subtree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    let mut res = 0;
    let mut count: i32 = 0;
    let mut s: i32 = 0;
    r(&root, &mut res, &mut count, &mut s);
    res
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_case1() {
        let root = &[4, 8, 5, 0, 1, i32::MIN, 6];
        assert_eq!(average_of_subtree(TreeNode::from_vec(root)), 5);
    }
    #[test]
    fn test_case2() {
        let root = &[1];
        assert_eq!(average_of_subtree(TreeNode::from_vec(root)), 1);
    }
}
