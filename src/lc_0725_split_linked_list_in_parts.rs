use crate::{ListNode, Solution};
impl Solution {
    pub fn split_list_to_parts(head: Option<Box<ListNode>>, k: i32) -> Vec<Option<Box<ListNode>>> {
        let mut length = 0;
        let mut current = head.as_ref();
        while let Some(node) = current {
            length += 1;
            current = node.next.as_ref();
        }

        let (base_size, mut extra) = (length / k, length % k);
        let mut current = head;
        let mut parts = Vec::new();
        for _ in 0..k {
            let mut part_size = if extra > 0 { base_size + 1 } else { base_size };
            let mut dummy = ListNode::new(0);
            let mut tail = &mut dummy;
            while part_size > 0 {
                tail.next = current.take();
                tail = tail.next.as_mut().unwrap();
                current = tail.next.take();
                part_size -= 1;
            }
            parts.push(dummy.next);
            extra -= 1;
        }

        parts
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let head = ListNode::from_vec(&[1, 2, 3]);
        let k = 5;
        let output = vec![
            ListNode::from_vec(&[1]),
            ListNode::from_vec(&[2]),
            ListNode::from_vec(&[3]),
            ListNode::from_vec(&[]),
            ListNode::from_vec(&[]),
        ];

        assert_eq!(Solution::split_list_to_parts(head, k), output);
    }

    #[test]
    fn test_case_2() {
        let head = ListNode::from_vec(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
        let k = 3;
        let output = vec![
            ListNode::from_vec(&[1, 2, 3, 4]),
            ListNode::from_vec(&[5, 6, 7]),
            ListNode::from_vec(&[8, 9, 10]),
        ];

        assert_eq!(Solution::split_list_to_parts(head, k), output);
    }
}
