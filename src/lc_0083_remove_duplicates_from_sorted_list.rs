use crate::ListNode;
use crate::Solution;

impl Solution {
    pub fn delete_duplicates_(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy = Some(Box::new(ListNode::new(101)));
        let mut prev = dummy.as_mut().unwrap();
        let mut cur = head;
        while let Some(mut node) = cur {
            cur = node.next.take();

            if prev.val != node.val {
                prev.next = Some(node);
                prev = prev.next.as_mut().unwrap();
            }
        }
        dummy.unwrap().next
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let head = ListNode::from_vec(&[1, 1, 2]);
        let res = ListNode::from_vec(&[1, 2]);
        assert_eq!(Solution::delete_duplicates_(head), res);
    }

    #[test]
    fn test_case_2() {
        let head = ListNode::from_vec(&[1, 1, 2, 3, 3]);
        let res = ListNode::from_vec(&[1, 2, 3]);
        assert_eq!(Solution::delete_duplicates_(head), res);
    }
}
