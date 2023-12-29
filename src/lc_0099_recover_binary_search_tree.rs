use std::{cell::RefCell, rc::Rc};

use crate::{Solution, TreeNode};

impl Solution {
    pub fn recover_tree(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        fn collect_in_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
            let mut stack = vec![];
            let mut answer = vec![];

            expand_left(root.clone(), &mut stack);
            while let Some(node) = stack.pop() {
                let node_ref = node.borrow();
                answer.push(node_ref.val);
                expand_left(node_ref.right.clone(), &mut stack);
            }
            answer
        }

        fn replace_in_order(root: Option<Rc<RefCell<TreeNode>>>, data: Vec<i32>) {
            let mut stack = vec![];
            let mut counter = 0;

            expand_left(root.clone(), &mut stack);
            while let Some(node) = stack.pop() {
                let mut node_ref = node.borrow_mut();
                node_ref.val = data[counter];
                counter += 1;
                expand_left(node_ref.right.clone(), &mut stack);
            }
        }

        fn expand_left(
            root: Option<Rc<RefCell<TreeNode>>>,
            stack: &mut Vec<Rc<RefCell<TreeNode>>>,
        ) {
            let mut node = root;
            while let Some(n) = node {
                node = n.borrow().left.clone();
                stack.push(n);
            }
        }

        let mut data = collect_in_order(root.clone());
        data.sort_unstable();
        replace_in_order(root.clone(), data);
    }
}

