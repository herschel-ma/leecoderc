use crate::ListNode;
use std::cmp::Ordering;
use std::collections::BinaryHeap;

impl PartialOrd for ListNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ListNode {
    fn cmp(&self, other: &Self) -> Ordering {
        self.val.cmp(&other.val).reverse()
    }
}

pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
    let mut pq = lists.into_iter().flatten().collect::<BinaryHeap<_>>();
    let mut head = None;
    let mut cur = &mut head;
    while let Some(node) = pq.pop() {
        cur = &mut cur.insert(Box::new(ListNode::new(node.val))).next;
        if let Some(next) = node.next {
            pq.push(next)
        }
    }
    head
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case1() {
        let lists = vec![
            ListNode::from_vec(&[1, 4, 5]),
            ListNode::from_vec(&[1, 3, 4]),
            ListNode::from_vec(&[2, 6]),
        ];
        let output = ListNode::from_vec(&[1, 1, 2, 3, 4, 4, 5, 6]);
        assert_eq!(merge_k_lists(lists), output);
    }

    #[test]
    fn test_case2() {
        let lists = vec![ListNode::from_vec(&[])];
        let output = ListNode::from_vec(&[]);
        assert_eq!(merge_k_lists(lists), output)
    }

    #[test]
    fn test_case3() {
        let lists = vec![None];
        let output = ListNode::from_vec(&[]);
        assert_eq!(merge_k_lists(lists), output)
    }
}
