use crate::ListNode;

// 我们先判断链表节点数是否小于2，如果是，真接返回 head 即可。
// 否则，我们先统计链表节点数 n，然后将k对n取模，得到k的有效值。
// 如果k的有效值为0，说明链表不需要旋转，直接返回head即可。
// 否则，我们用快慢指针，让快指针先走k
// 步，然后快慢指针同时行，直到快慢指针走到链表尾部，此时慢指针的下一节点就是新的链表头节点。
// 最后，我们将链表拼接起来即可。
// 时间复杂度:O(n), 其中n是链表的节点数，空间复杂度 O(1)。
pub fn rotate_right_solution_1(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
    let mut head = head;
    if head.is_none() || k == 0 {
        return head;
    }
    let n = {
        let mut cur = &head;
        let mut res = 0;
        while cur.is_some() {
            res += 1;
            cur = &cur.as_ref().unwrap().next;
        }
        res
    };

    let mut k = k;
    k %= n;
    if k == 0 {
        return head;
    }
    let mut cur = head.as_mut();
    for _ in 0..n - k - 1 {
        cur = cur.unwrap().next.as_mut();
    }
    let mut new_head = cur.unwrap().next.take();
    // find the end of new_head and connect it to old head
    let mut cur = new_head.as_mut();
    while let Some(node) = cur {
        if node.next.is_none() {
            node.next = head;
            break;
        }
        cur = node.next.as_mut();
    }
    new_head
}

pub fn rotate_right_solution_2(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
    let mut head = head;
    if head.is_none() || k == 0 {
        return head;
    }
    let mut sz = 0;
    let mut cur = head.as_ref();
    while let Some(node) = cur {
        sz += 1;
        cur = node.next.as_ref();
    }
    let cutoff = sz - k % sz;
    if cutoff == sz {
        return head;
    }
    let mut cur = head.as_mut().unwrap();
    for _ in 0..(cutoff - 1) {
        cur = cur.next.as_mut().unwrap();
    }

    let mut new_head = cur.next.take();
    let mut cur = new_head.as_mut();
    while let Some(node) = cur {
        if node.next.is_none() {
            node.next = head;
            break;
        }
        cur = node.next.as_mut();
    }
    new_head
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let solution: fn(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> =
            rotate_right_solution_2;
        let head = ListNode::from_vec(&[1, 2, 3, 4, 5]);
        let k = 2;
        assert_eq!(solution(head, k), ListNode::from_vec(&[4, 5, 1, 2, 3]));
    }

    #[test]
    fn test_case_2() {
        let solution: fn(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> =
            rotate_right_solution_2;
        let head = ListNode::from_vec(&[0, 1, 2]);
        let k = 4;
        assert_eq!(solution(head, k), ListNode::from_vec(&[2, 0, 1]));
    }
}
