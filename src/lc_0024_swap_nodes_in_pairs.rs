use crate::ListNode;

pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    match head {
        Some(mut first) => {
            if let Some(mut second) = first.next.take() {
                first.next = swap_pairs(second.next.take());
                second.next = Some(first);
                Some(second)
            } else {
                Some(first)
            }
        }
        None => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case1() {
        let head = ListNode::from_vec(&[1, 2, 3, 4]);
        let output = ListNode::from_vec(&[2, 1, 4, 3]);
        assert_eq!(swap_pairs(head), output);
    }

    #[test]
    fn test_case2() {
        let head = ListNode::from_vec(&[]);
        let output = ListNode::from_vec(&[]);
        assert_eq!(swap_pairs(head), output);
    }

    #[test]
    fn test_case3() {
        let head = ListNode::from_vec(&[1]);
        let output = ListNode::from_vec(&[1]);
        assert_eq!(swap_pairs(head), output)
    }
}

