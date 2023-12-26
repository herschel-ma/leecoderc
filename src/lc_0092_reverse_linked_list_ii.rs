use crate::{ListNode, Solution};

impl Solution {
    pub fn reverse_between(
        head: Option<Box<ListNode>>,
        left: i32,
        right: i32,
    ) -> Option<Box<ListNode>> {
        let mut dummy = Some(Box::new(ListNode { val: 0, next: head }));
        let mut pre = &mut dummy;
        for _ in 0..left - 1 {
            pre = &mut pre.as_mut()?.next;
        }
        let mut node = pre.as_mut()?.next.take();
        for _ in 0..right - left + 1 {
            let next = node.as_mut()?.next.take();
            node.as_mut()?.next = pre.as_mut()?.next.take();
            pre.as_mut()?.next = node.take();
            node = next;
        }

        for _ in 0..right - left + 1 {
            pre = &mut pre.as_mut()?.next;
        }
        pre.as_mut()?.next = node;

        dummy.unwrap().next
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let head = ListNode::from_vec(&[1, 2, 3, 4, 5]);
        let left = 2;
        let right = 4;
        let res = ListNode::from_vec(&[1, 4, 3, 2, 5]);
        assert_eq!(Solution::reverse_between(head, left, right), res);
    }

    #[test]
    fn test_case_2() {
        let head = ListNode::from_vec(&[5]);
        let left = 1;
        let right = 1;
        let res = ListNode::from_vec(&[5]);
        assert_eq!(Solution::reverse_between(head, left, right), res);
    }
}
