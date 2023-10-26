use crate::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
use std::collections::VecDeque;

//BFS
pub fn largest_values(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut result = Vec::new();
    if let Some(root) = root {
        let mut queue = VecDeque::with_capacity(1000);
        queue.push_back(root);
        while !queue.is_empty() {
            let level_size = queue.len();
            let mut max_value = i32::MIN;
            for _ in 0..level_size {
                if let Some(node) = queue.pop_front() {
                    let node = node.borrow();
                    max_value = max_value.max(node.val);
                    if let Some(left) = node.left.clone() {
                        queue.push_back(left)
                    }
                    if let Some(right) = node.right.clone() {
                        queue.push_back(right)
                    }
                }
            }
            result.push(max_value);
        }
    }
    result
}


#[cfg(test)]
mod tests {
    use crate::{largest_values, TreeNode};

    #[test]
    fn case() {
        let input = vec![1, 3, 2, 5, 3, i32::MIN, 9];
        let root =  TreeNode::from_vec(input.as_slice()) ;
        let output = vec![1, 3, 9];
        assert_eq!(largest_values(root), output);
    }
    
    #[test]
    fn case2() {
        let input = vec![1, 2, 3];
        let root =  TreeNode::from_vec(input.as_slice()) ;
        let output = vec![1, 3];
        assert_eq!(largest_values(root), output);
    }
}

