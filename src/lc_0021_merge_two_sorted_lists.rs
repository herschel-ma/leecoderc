use crate::ListNode;
pub fn merge_two_lists(
    list1: Option<Box<ListNode>>,
    list2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    if list1.is_none() {
        return list2;
    }

    if list2.is_none() {
        return list1;
    }
    let mut vec_result = Vec::new();
    let mut l1 = list2;
    let mut l2 = list1;

    while l1.is_some() && l2.is_some() {
        let l1ref = l1.as_ref().unwrap().val;
        let l2ref = l2.as_ref().unwrap().val;
        if l1ref < l2ref {
            vec_result.push(l1ref);
            l1 = l1.unwrap().next;
        } else {
            vec_result.push(l2ref);
            l2 = l2.unwrap().next;
        }
    }

    while let Some(entry) = l1 {
        vec_result.push(entry.val);
        l1 = entry.next;
    }

    while let Some(entry) = l2 {
        vec_result.push(entry.val);
        l2 = entry.next;
    }

    ListNode::from_vec(&vec_result)
}

mod tests {

    #[test]
    fn ex1() {
        use crate::{merge_two_lists, ListNode};
        let list1 = ListNode::from_vec(&vec![1, 2, 4]);
        let list2 = ListNode::from_vec(&vec![1, 3, 4]);
        let check = vec![1, 1, 2, 3, 4, 4];
        let mut res = merge_two_lists(list1, list2);
        let mut n = 0;
        while let Some(entry) = res {
            assert_eq!(entry.val, check[n]);
            res = entry.next;
            n += 1;
        }
        assert_eq!(n, check.len());
    }

    #[test]
    fn ex2() {
        use crate::merge_two_lists;
        let merged = merge_two_lists(None, None);
        assert!(merged.is_none());
    }

    #[test]
    fn ex3() {
        use crate::{merge_two_lists, ListNode};
        let list2 = Some(Box::new(ListNode::new(0)));
        let merged = merge_two_lists(None, list2);
        assert_eq!(merged.as_ref().unwrap().val, 0);
        assert!(merged.as_ref().unwrap().next.is_none());
    }
}
