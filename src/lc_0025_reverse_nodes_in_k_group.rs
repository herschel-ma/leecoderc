use crate::ListNode;

pub fn reverse_k_group_1(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
    fn reverse(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut head = head;
        let mut pre = None;
        while let Some(mut node) = head {
            head = node.next.take();
            node.next = pre;
            pre = Some(node);
        }
        pre
    }
    let mut dummy = Some(Box::new(ListNode::new(0)));
    let mut pre = &mut dummy;
    let mut cur = head;
    while cur.is_some() {
        let mut q = &mut cur;
        for _ in 0..k - 1 {
            if q.is_none() {
                break;
            }
            q = &mut q.as_mut().unwrap().next
        }
        if q.is_none() {
            pre.as_mut().unwrap().next = cur;
            return dummy.unwrap().next;
        }
        let b = q.as_mut().unwrap().next.take();
        pre.as_mut().unwrap().next = reverse(cur);
        while pre.is_some() && pre.as_mut().unwrap().next.is_some() {
            pre = &mut pre.as_mut().unwrap().next;
        }
        cur = b
    }
    dummy.unwrap().next
}

pub fn reverse_k_group(mut head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
    let mut new_group_head = &mut head;
    // 查看剩余部分长度是否大于等于 k
    for _ in 0..k {
        if new_group_head.is_none() {
            return head;
        }
        new_group_head = &mut new_group_head.as_mut().unwrap().next;
    }
    // 递归翻转后面
    let mut new_head = reverse_k_group(new_group_head.take(), k);
    // 将前半部分逐个拼到 new_head (已经翻转好的后半部分) 的前面
    while head.is_some() {
        let node = head.as_mut().unwrap();
        let next = node.next.take();
        node.next = new_head.take();
        new_head = head;
        head = next;
    }

    new_head
}

#[allow(dead_code)]
fn print_list(list: &Option<Box<ListNode>>) {
    let mut current = list;
    while let Some(node) = current {
        print!("{} -> ", node.val);
        current = &node.next;
    }
    println!("None");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case1() {
        let head = ListNode::from_vec(&[1, 2, 3, 4, 5]);
        let k = 2;
        let output = ListNode::from_vec(&[2, 1, 4, 3, 5]);
        assert_eq!(reverse_k_group(head, k), output)
    }

    #[test]
    fn test_case2() {
        let head = ListNode::from_vec(&[1, 2, 3, 4, 5]);
        let k = 3;
        let output = ListNode::from_vec(&[3, 2, 1, 4, 5]);
        assert_eq!(reverse_k_group(head, k), output)
    }
    #[test]
    fn test_case() {
        let head = ListNode::from_vec(&[1, 2, 3, 4]);
        let k = 1;
        let output = ListNode::from_vec(&[1, 2, 3, 4]);
        assert_eq!(reverse_k_group(head, k), output)
    }

    #[test]
    fn test_case3() {
        let mut linked_list = Some(Box::new(ListNode::new(1)));
        let mut current = &mut linked_list;
        for i in 2..=5 {
            current.as_mut().unwrap().next = Some(Box::new(ListNode::new(i)));
            current = &mut current.as_mut().unwrap().next;
        }
        // 调用 reverse_k_group
        let k = 3;
        let reversed_list = reverse_k_group(linked_list, k);
        // 打印结果
        print_list(&reversed_list);
    }
}
