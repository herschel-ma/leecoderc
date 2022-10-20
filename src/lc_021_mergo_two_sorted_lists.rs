use crate::ListNode;
pub fn merge_two_lists(list1: Option<Box<ListNode>>, list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    None
}



mod tests {
    use crate::{ListNode, merge_two_lists};

    #[test]
    fn ex1() {
        let list1 = ListNode::from_vec(&vec![1,2,4]);
        let list2 = ListNode::from_vec(&vec![1,3,4]);
        let check = vec![1,1,2,3,4,4];
        let mut res = merge_two_lists(list1, list2);
        let mut n = 0;
        while let Some(entry) = res {
            assert_eq!(entry.val, check[n]);
            res = entry.next;
            n += 1;
        }
        assert_eq!(n, check.len());
    }
}