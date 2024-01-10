use crate::{Solution, TreeNode};
use std::cell::RefCell;
use std::collections::{HashMap, HashSet, VecDeque};
use std::rc::Rc;

impl Solution {
    pub fn amount_of_time(root: Option<Rc<RefCell<TreeNode>>>, start: i32) -> i32 {
        fn collect_gragh(root: &Option<Rc<RefCell<TreeNode>>>, graph: &mut HashMap<i32, Vec<i32>>) {
            if let Some(node) = root {
                let node = node.borrow();
                let left = node.left.as_ref();
                let right = node.right.as_ref();
                let val = node.val;
                graph.entry(val).or_default().push(-1);
                if let Some(left) = left {
                    graph.entry(val).or_default().push(left.borrow().val);
                    graph.entry(left.borrow().val).or_default().push(val);
                    collect_gragh(&node.left, graph);
                }

                if let Some(right) = right {
                    graph.entry(val).or_default().push(right.borrow().val);
                    graph.entry(right.borrow().val).or_default().push(val);
                    collect_gragh(&node.right, graph);
                }
            }
        }

        fn bfs(graph: &HashMap<i32, Vec<i32>>, start: i32, res: &mut i32) {
            let mut queue = VecDeque::<i32>::new();
            let mut visited = HashSet::<i32>::new();

            queue.push_back(start);
            visited.insert(start);

            while !queue.is_empty() {
                let size = queue.len();

                for _ in 0..size {
                    if let Some(n) = queue.pop_front() {
                        if let Some(mp) = graph.get(&n) {
                            for &m in mp {
                                if !visited.insert(m) || m == -1 {
                                    continue;
                                }
                                queue.push_back(m);
                            }
                        }
                    }
                }
                (!queue.is_empty()).then(|| {
                    *res += 1;
                });
            }
        }

        let mut ans = 0;
        let mut g: HashMap<i32, Vec<i32>> = HashMap::new();

        collect_gragh(&root, &mut g);
        bfs(&g, start, &mut ans);
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let root = TreeNode::from_vec(&[1, 5, 3, i32::MIN, 4, 10, 6, 9, 2]);
        let start = 3;
        assert_eq!(Solution::amount_of_time(root, start), 4);
    }

    #[test]
    fn test_case_2() {
        let root = TreeNode::from_vec(&[1]);
        let start = 1;
        assert_eq!(Solution::amount_of_time(root, start), 0);
    }
}

