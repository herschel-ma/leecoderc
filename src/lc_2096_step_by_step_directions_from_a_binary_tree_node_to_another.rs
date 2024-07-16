use crate::{Solution, TreeNode};
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn get_directions(
        root: Option<Rc<RefCell<TreeNode>>>,
        start_value: i32,
        dest_value: i32,
    ) -> String {
        // 1. find path from root to start
        // 2. find path from toot to dest
        // 3. remove dup parent nodes
        // 4. revert path to start, and join path with dest
        let mut path_to_start: Vec<char> = vec![];
        let mut path_to_dest: Vec<char> = vec![];

        Self::find_path(&root, start_value, &mut path_to_start);
        Self::find_path(&root, dest_value, &mut path_to_dest);

        let mut start_idx = 0;
        for i in 0..(path_to_start.len()).min(path_to_dest.len()) {
            if path_to_start[i] == path_to_dest[i] {
                start_idx = i + 1;
            } else {
                break;
            }
        }

        path_to_start = path_to_start[start_idx..].to_vec();
        path_to_dest = path_to_dest[start_idx..].to_vec();

        // revert path_to_start
        let mut path_to_start_rev = vec!['U'; path_to_start.len()];
        // join with dest
        path_to_start_rev.append(&mut path_to_dest);
        path_to_start_rev.iter().collect()
    }

    pub fn find_path(
        root: &Option<Rc<RefCell<TreeNode>>>,
        value: i32,
        path: &mut Vec<char>,
    ) -> bool {
        match root {
            None => false,
            Some(node) => {
                if node.borrow().val == value {
                    true
                } else {
                    path.push('L');
                    if Self::find_path(&node.borrow().left, value, path) {
                        return true;
                    }
                    path.pop();
                    path.push('R');
                    if Self::find_path(&node.borrow().right, value, path) {
                        return true;
                    }
                    path.pop();
                    false
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let root = TreeNode::from_vec(&[5, 1, 2, 3, i32::MIN, 6, 4]);
        let start_value = 3;
        let dest_value = 6;
        assert_eq!(
            Solution::get_directions(root, start_value, dest_value),
            "UURL".to_string()
        );
    }

    #[test]
    fn test_case_2() {
        let root = TreeNode::from_vec(&[2, 1]);
        let start_value = 2;
        let dest_value = 1;
        assert_eq!(
            Solution::get_directions(root, start_value, dest_value),
            "L".to_string()
        );
    }
}
