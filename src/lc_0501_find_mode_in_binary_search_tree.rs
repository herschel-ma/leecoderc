use crate::tree::TreeNode;
use std::collections::HashMap;
use std::{cell::RefCell, rc::Rc};
fn r(root: &Option<Rc<RefCell<TreeNode>>>, freq: &mut HashMap<i32, i32>) {
    if let Some(root) = root {
        let node = root.borrow();
        let value = node.val;
        *freq.entry(value).or_insert(0) += 1;
        r(&&node.left, freq);
        r(&&node.right, freq);
    }
}

pub fn find_mode(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut res: Vec<i32> = Vec::new();
    let mut freq: HashMap<i32, i32> = HashMap::new();
    r(&root, &mut freq);
    let mut v: Vec<_> = freq.iter().collect();
    v.sort_by(|a, b| b.1.cmp(a.1));
    println!("{:?}", v);
    let mx = v[0].1;
    for (k, val) in v {
        println!("{}", mx);
        if mx == val {
            res.push(*k)
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;
    #[cfg(ignore)]
    #[test]
    fn case1() {
        let root = &[1, i32::MIN, 2, 2];
        let tree = TreeNode::from_vec(root);
        let res = vec![2];
        assert_eq!(find_mode(tree), res)
    }

    #[test]
    fn case2() {
        let root = &[0];
        let tree = TreeNode::from_vec(root);
        let res = vec![0];
        assert_eq!(find_mode(tree), res)
    }
}
