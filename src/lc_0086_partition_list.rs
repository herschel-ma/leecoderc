use crate::{ListNode, Solution};

impl Solution {
    pub fn partition(head: Option<Box<ListNode>>, x: i32) -> Option<Box<ListNode>> {
        let mut head = head;
        let mut d1 = Some(Box::new(ListNode::new(0)));
        let mut d2 = Some(Box::new(ListNode::new(0)));
        let (mut ptr1, mut ptr2) = (&mut d1, &mut d2);
        while let Some(mut node) = head {
            head = node.next.take();
            if node.val < x {
                ptr1.as_mut().unwrap().next = Some(node);
                ptr1 = &mut ptr1.as_mut().unwrap().next;
            } else {
                ptr2.as_mut().unwrap().next = Some(node);
                ptr2 = &mut ptr2.as_mut().unwrap().next;
            }
        }
        ptr1.as_mut().unwrap().next = d2.unwrap().next;

        d1.unwrap().next
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let head = ListNode::from_vec(&[1, 4, 3, 2, 5, 2]);
        let x = 3;
        assert_eq!(
            Solution::partition(head, x),
            ListNode::from_vec(&[1, 2, 2, 4, 3, 5])
        );
    }

    #[test]
    fn test_case_2() {
        let head = ListNode::from_vec(&[2, 1]);
        let x = 2;
        assert_eq!(Solution::partition(head, x), ListNode::from_vec(&[1, 2]));
    }
}
