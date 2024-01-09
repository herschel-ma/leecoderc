use crate::{ListNode, Solution};

impl Solution {
    pub fn reorder_list(head: &mut Option<Box<ListNode>>) {
        fn get_list_middle(head: &mut Option<Box<ListNode>>) -> Option<Box<ListNode>> {
            let (mut fast, mut slow) = (&head.clone(), head);
            while fast.is_some() {
                fast = &fast.as_ref().unwrap().next;
                if fast.is_some() {
                    fast = &fast.as_ref().unwrap().next;
                    slow = &mut slow.as_mut().unwrap().next;
                }
            }
            slow.as_mut().unwrap().next.take()
        }

        fn reverse_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
            let mut pre = None;
            while let Some(mut cur) = head {
                head = cur.next;
                cur.next = pre;
                pre = Some(cur);
            }
            pre
        }

        fn merge_list(head1: &mut Option<Box<ListNode>>, head2: Option<Box<ListNode>>) {
            let mut h1 = head1;
            let mut h2 = head2;
            while h1.is_some() && h2.is_some() {
                let h1next = h1.as_mut().unwrap().next.take();
                let h2next = h2.as_mut().unwrap().next.take();
                h1.as_mut().unwrap().next = h2;
                h1.as_mut().unwrap().next.as_mut().unwrap().next = h1next;
                h1 = &mut h1.as_mut().unwrap().next.as_mut().unwrap().next;
                h2 = h2next;
            }
        }

        let mut head2 = get_list_middle(head);
        head2 = reverse_list(head2);
        merge_list(head, head2);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let mut head = ListNode::from_vec(&[1, 2, 3, 4]);
        Solution::recorder_list(&mut head);
        assert_eq!(head, ListNode::from_vec(&[1, 4, 2, 3]));
    }

    #[test]
    fn test_case_2() {
        let mut head = ListNode::from_vec(&[1, 2, 3, 4, 5]);
        Solution::recorder_list(&mut head);
        assert_eq!(head, ListNode::from_vec(&[1, 5, 2, 4, 3]));
    }
}

