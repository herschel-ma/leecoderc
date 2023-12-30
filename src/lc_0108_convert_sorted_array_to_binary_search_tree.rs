use crate::{Solution, TreeNode};
use std::{cell::RefCell, rc::Rc};

impl Solution {
    pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        fn build(nums: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
            if nums.is_empty() {
                return None;
            }

            let mid = nums.len() >> 1;

            let tree_node = Rc::new(RefCell::new(TreeNode::new(nums[mid])));
            tree_node.borrow_mut().left = build(&nums[..mid]);
            tree_node.borrow_mut().right = build(&nums[mid + 1..]);

            Some(tree_node)
        }

        build(&nums[..])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let nums = vec![-10, -3, 0, 5, 9];
        let candidates = vec![
            TreeNode::from_vec(&[0, -3, 9, -10, i32::MIN, 5]),
            TreeNode::from_vec(&[0, -10, 5, i32::MIN, -3, i32::MIN, 9]),
        ];
        let res = Solution::sorted_array_to_bst(nums);
        assert!(candidates.contains(&res));
    }

    #[test]
    fn test_case_2() {
        let nums = vec![1, 3];
        let candidates = vec![
            TreeNode::from_vec(&[3, 1]),
            TreeNode::from_vec(&[1, i32::MIN, 3]),
        ];

        let res = Solution::sorted_array_to_bst(nums);
        assert!(candidates.contains(&res));
    }
}

