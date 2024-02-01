fn is_binary_tree_valid(
    current: i32,
    left_child: &Vec<i32>,
    right_child: &Vec<i32>,
    vistied: &mut Vec<bool>,
) -> bool {
    // check left child
    if left_child[current as usize] != -1 {
        if vistied[left_child[current as usize] as usize] {
            // check for cycle
            return false;
        }
        // mark left child as visited
        vistied[left_child[current as usize] as usize] = true;
        if !is_binary_tree_valid(
            left_child[current as usize],
            left_child,
            right_child,
            vistied,
        ) {
            return false;
        }
    }

    // check right child
    if right_child[current as usize] != -1 {
        if vistied[right_child[current as usize] as usize] {
            return false;
        }
        // mark right chidl as visited
        vistied[right_child[current as usize] as usize] = true;
        if !is_binary_tree_valid(
            right_child[current as usize],
            left_child,
            right_child,
            vistied,
        ) {
            return false;
        }
    }
    true
}

pub fn validate_binary_tree_nodes(n: i32, left_child: Vec<i32>, right_child: Vec<i32>) -> bool {
    let mut child_count = vec![false; n as usize];

    for child in left_child.iter() {
        if *child != -1 {
            // mark left child as hvaing a parent
            child_count[*child as usize] = true;
        }
    }

    for child in right_child.iter() {
        if *child != -1 {
            // current node already has a parent
            if child_count[*child as usize] {
                return false;
            }
            // mark right child as hvaing a parent
            child_count[*child as usize] = true;
        }
    }

    // find if has multiple roots
    let mut root = -1;
    for i in 0..n {
        if !child_count[i as usize] {
            if root == -1 {
                root = i;
            } else {
                // multiple roots found, not a valid binary tree.
                return false;
            }
        }
    }
    // not found root, not a valid binary tree
    if root == -1 {
        return false;
    }

    let mut visited = vec![false; n as usize];
    visited[root as usize] = true;
    if !is_binary_tree_valid(root, &left_child, &right_child, &mut visited) {
        return false;
    }

    // check if there is multiple components
    for visit in visited {
        if !visit {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let n = 4;
        let left_child = vec![1, -1, 3, -1];
        let right_child = vec![2, -1, -1, -1];
        assert!(validate_binary_tree_nodes(n, left_child, right_child))
    }

    #[test]
    fn case2() {
        let n = 4;
        let left_child = vec![1, -1, 3, -1];
        let right_child = vec![2, 3, -1, -1];
        assert!(!validate_binary_tree_nodes(n, left_child, right_child))
    }

    #[test]
    fn case3() {
        let n = 2;
        let left_child = vec![1, 0];
        let right_child = vec![-1, -1];
        assert!(!validate_binary_tree_nodes(n, left_child, right_child))
    }
}
