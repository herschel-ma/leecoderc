use crate::ListNode;
pub fn reverse_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut ary = vec![];
    while let Some(node) = head {
        ary.push(node.val);
        head = node.next;
    }
    let mut ptr = None;
    for val in ary {
        let mut node = ListNode::new(val);
        node.next = ptr;
        ptr = Some(Box::new(node));
    }
    ptr
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let head = ListNode::from_vec(&vec![1, 2, 3, 4, 5]);
        assert_eq!(
            vec![5, 4, 3, 2, 1],
            reverse_list(head).unwrap().as_ref().into_array()
        )
    }

    #[test]
    fn ex2() {
        let head = ListNode::from_vec(&vec![1, 2]);
        assert_eq!(
            vec![2, 1],
            reverse_list(head).unwrap().as_ref().into_array()
        )
    }

    #[test]
    fn ex3() {
        assert!(reverse_list(None).is_none())
    }
}
