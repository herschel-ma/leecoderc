use crate::TreeNode;
use std::{cell::RefCell, rc::Rc};

pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    height_is_balanced(&root).is_some()
}

fn height_is_balanced(root: &Option<Rc<RefCell<TreeNode>>>) -> Option<i32> {
    if let Some(root) = root {
        let lh = height_is_balanced(&root.borrow().left);
        let rh = height_is_balanced(&root.borrow().right);
        match (lh, rh) {
            (Some(lh), Some(rh)) => {
                if (lh - rh).abs() < 2 {
                    Some(lh.max(rh) + 1)
                } else {
                    None
                }
            }
            _ => None,
        }
    } else {
        Some(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn ex1() {
        let tree = TreeNode::from_vec(&[3, 9, 20, i32::MIN, i32::MIN, 15, 7]);
        assert!(is_balanced(tree));
    }

    #[test]
    fn ex2() {
        let tree = TreeNode::from_vec(&[1, 2, 2, 3, 3, i32::MIN, i32::MIN, 4, 4]);
        assert!(!is_balanced(tree));
    }

    #[test]
    fn ex3() {
        let tree = TreeNode::from_vec(&[]);
        assert!(is_balanced(tree));
    }
}
