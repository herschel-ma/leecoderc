use crate::{Solution, TreeNode};
use std::cell::RefCell;
use std::rc::Rc;

type Node = Option<Rc<RefCell<TreeNode>>>;

impl Solution {
    pub fn generate_trees(n: i32) -> Vec<Node> {
        fn dfs(l: i32, r: i32) -> Vec<Node> {
            if l > r {
                return vec![None];
            }
            let mut a = vec![];
            for i in l..=r {
                let left = dfs(l, i - 1);
                let right = dfs(i + 1, r);
                for l in left {
                    for r in &right {
                        let mut root = TreeNode::new(i);
                        root.left = l.clone();
                        root.right = r.clone();
                        a.push(Some(Rc::new(RefCell::new(root))));
                    }
                }
            }
            a
        }

        dfs(1, n)
    }
}
