use crate::ListNode;

pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut fast = &head;
    let mut slow = &head;

    let mut flip_slow = false;
    while let Some(node) = fast {
        fast = &node.next;
        if flip_slow {
            slow = &slow.as_ref().unwrap().next;
        }
        flip_slow = !flip_slow
        // [ 1 1 1 1 1 1 1]
        //   s s s s
        //   f f f f f f f
    }
    slow.clone()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let list = ListNode::from_vec(&[1, 2, 3, 4, 5]);
        let middle = middle_node(list).unwrap().into_array();
        assert_eq!(middle, vec![3, 4, 5]);
    }
    #[test]
    fn ex2() {
        let list = ListNode::from_vec(&[1, 2, 3, 4, 5, 6]);
        let middle = middle_node(list).unwrap().into_array();
        assert_eq!(middle, vec![4, 5, 6]);
    }
}
