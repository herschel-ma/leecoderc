use crate::ListNode;

pub fn merge_two_lists_1(
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

pub fn merge_two_lists_2(
    list1: Option<Box<ListNode>>,
    list2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    match (list1, list2) {
        (None, None) => None,
        (None, Some(list)) => Some(list),
        (Some(list), None) => Some(list),
        (Some(mut list1), Some(mut list2)) => {
            if list1.val < list2.val {
                list1.next = merge_two_lists(list1.next, Some(list2));
                Some(list1)
            } else {
                list2.next = merge_two_lists(Some(list1), list2.next);
                Some(list2)
            }
        }
    }
}

pub fn merge_two_lists(
    mut list1: Option<Box<ListNode>>,
    mut list2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let mut new_list = ListNode::new(0);
    let mut cur = &mut new_list;
    while list1.is_some() && list2.is_some() {
        let (l1, l2) = (list1.as_deref_mut().unwrap(), list2.as_deref_mut().unwrap());
        if l1.val < l2.val {
            let next = l1.next.take();
            cur.next = list1.take();
            list1 = next;
        } else {
            let next = l2.next.take();
            cur.next = list2.take();
            list2 = next;
        }
        cur = cur.next.as_deref_mut().unwrap();
    }
    cur.next = list1.or(list2);
    new_list.next
}

mod tests {

    #[test]
    fn ex1() {
        use crate::{merge_two_lists, ListNode};
        let list1 = ListNode::from_vec(&[1, 2, 4]);
        let list2 = ListNode::from_vec(&[1, 3, 4]);
        let check = [1, 1, 2, 3, 4, 4];
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
