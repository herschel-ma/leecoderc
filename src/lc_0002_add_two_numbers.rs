use crate::ListNode;

pub fn add_two_numbers_v1(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let mut carray = 0;
    let mut l1 = l1;
    let mut l2 = l2;
    let mut dummy = ListNode::new(-1);
    let mut p = &mut dummy;
    while l1.is_some() || l2.is_some() {
        let mut sum = carray;
        if let Some(node) = l1 {
            sum += node.val;
            l1 = node.next;
        }
        if let Some(node) = l2 {
            sum += node.val;
            l2 = node.next;
        }
        carray = sum / 10;
        p.next = Some(Box::new(ListNode {
            val: sum % 10,
            next: None,
        }));
        p = p.next.as_mut().unwrap();
    }
    if carray != 0 {
        p.next = Some(Box::new(ListNode {
            val: carray,
            next: None,
        }));
    }
    dummy.next
}

pub fn add_two_numbers(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    match (l1, l2) {
        (None, None) => None,
        (Some(n), None) | (None, Some(n)) => Some(n),
        (Some(n1), Some(n2)) => {
            let sum = n1.val + n2.val;
            if sum < 10 {
                Some(Box::new(ListNode {
                    val: sum,
                    next: add_two_numbers(n1.next, n2.next),
                }))
            } else {
                let carry = Some(Box::new(ListNode::new(1)));
                Some(Box::new(ListNode {
                    val: sum - 10,
                    next: add_two_numbers(add_two_numbers(carry, n1.next), n2.next),
                }))
            }
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case1() {
        let l1 = ListNode::from_vec(&[2, 4, 3]);
        let l2 = ListNode::from_vec(&[5, 6, 4]);
        let output = vec![7, 0, 8];
        assert_eq!(add_two_numbers(l1, l2), ListNode::from_vec(&output));
    }

    #[test]
    fn test_case2() {
        let l1 = ListNode::from_vec(&[0]);
        let l2 = ListNode::from_vec(&[0]);
        let output = vec![0];
        assert_eq!(add_two_numbers(l1, l2), ListNode::from_vec(&output));
    }

    #[test]
    fn test_case3() {
        let l1 = ListNode::from_vec(&[9, 9, 9, 9, 9, 9, 9]);
        let l2 = ListNode::from_vec(&[9, 9, 9, 9]);
        let output = vec![8, 9, 9, 9, 0, 0, 0, 1];
        assert_eq!(add_two_numbers(l1, l2), ListNode::from_vec(&output));
    }
}

