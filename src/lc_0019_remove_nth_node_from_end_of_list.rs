use crate::ListNode;

pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
    let mut dummy = Some(Box::new(ListNode { val: 0, next: head }));
    let mut back = &mut dummy;
    let mut front = &back.clone();
    for _ in 0..=n {
        front = &front.as_ref().unwrap().next;
    }
    while front.is_some() {
        front = &front.as_ref().unwrap().next;
        back = &mut back.as_mut().unwrap().next;
    }
    back.as_mut().unwrap().next = back.as_mut().unwrap().next.as_mut().unwrap().next.take();
    dummy.unwrap().next
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case1() {
        let head = ListNode::from_vec(&[1, 2, 3, 4, 5]);
        let n = 2;
        let output = ListNode::from_vec(&[1, 2, 3, 5]);
        assert_eq!(remove_nth_from_end(head, n), output)
    }

    #[test]
    fn test_case2() {
        let head = ListNode::from_vec(&[1, 2]);
        let n = 1;
        let output = ListNode::from_vec(&[1]);
        assert_eq!(remove_nth_from_end(head, n), output)
    }
}

