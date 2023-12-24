use crate::{ListNode, Solution};

impl Solution {
    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy = Some(Box::new(ListNode::new(101)));
        let mut cur = head;
        let mut pev = dummy.as_mut().unwrap();
        let mut pre = pev.val;
        while let Some(mut node) = cur {
            cur = node.next.take(); // node.next = None
            if pre == node.val || (cur.is_some() && cur.as_ref().unwrap().val == node.val) {
                pre = node.val;
            } else {
                pre = node.val;
                pev.next = Some(node);
                pev = pev.next.as_mut().unwrap();
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
        let head = ListNode::from_vec(&[1, 2, 3, 3, 4, 4, 5]);
        let output = ListNode::from_vec(&[1, 2, 5]);
        assert_eq!(Solution::delete_duplicates(head), output);
    }

    #[test]
    fn test_case_2() {
        let head = ListNode::from_vec(&[1, 1, 1, 2, 3]);
        let output = ListNode::from_vec(&[2, 3]);
        assert_eq!(Solution::delete_duplicates(head), output);
    }
}
