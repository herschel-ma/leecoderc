use crate::{Solution, TreeNode};
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;

impl Solution {
    pub fn create_binary_tree(descriptions: Vec<Vec<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut nodes = HashMap::new();
        let mut children = HashSet::new();
        for description in descriptions.iter() {
            let (parent, child, is_left) = (description[0], description[1], description[2]);
            nodes
                .entry(parent)
                .or_insert(Rc::new(RefCell::new(TreeNode::new(parent))));

            nodes
                .entry(child)
                .or_insert(Rc::new(RefCell::new(TreeNode::new(child))));

            if is_left == 1 {
                nodes.get_mut(&parent).unwrap().borrow_mut().left = nodes.get(&child).cloned();
            } else {
                nodes.get_mut(&parent).unwrap().borrow_mut().right = nodes.get(&child).cloned();
            }
            children.insert(child);
        }

        for description in descriptions.iter() {
            let (parent, _, _) = (description[0], description[1], description[2]);
            if !children.contains(&parent) {
                return nodes.get(&parent).cloned();
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let descriptions = vec![
            vec![20, 15, 1],
            vec![20, 17, 0],
            vec![50, 20, 1],
            vec![50, 80, 0],
            vec![80, 19, 1],
        ];
        let output = TreeNode::from_vec(&[50, 20, 80, 15, 17, 19]);
        assert_eq!(Solution::create_binary_tree(descriptions), output);
    }

    #[test]
    fn test_case_2() {
        let descriptions = vec![vec![1, 2, 1], vec![2, 3, 0], vec![3, 4, 1]];
        let output = TreeNode::from_vec(&[1, 2, i32::MIN, i32::MIN, 3, 4]);
        assert_eq!(Solution::create_binary_tree(descriptions), output);
    }
}
