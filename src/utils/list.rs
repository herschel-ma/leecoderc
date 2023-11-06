// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }

    pub fn from_vec(vec: &[i32]) -> Option<Box<ListNode>> {
        let mut result = None;
        for i in vec.iter().rev() {
            let mut node = Self::new(*i);
            node.next = result;
            // println!("i = {i:?}, node={node:?}");
            result = Some(Box::new(node));
        }
        result
    }

    pub fn into_array(&self) -> Vec<i32> {
        let mut res = vec![self.val];
        let mut head = &self.next;
        while let Some(node) = head {
            res.push(node.val);
            head = &node.next;
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    #[test]
    pub fn creat_list() {
        let mut list = ListNode::from_vec(&[1, 2, 3, 4, 5]);

        let mut n = 1;
        while let Some(entry) = list {
            assert_eq!(entry.val, n);
            n += 1;
            list = entry.next
        }
    }

    #[test]
    fn convert_back_to_array() {
        let list = ListNode::from_vec(&[1, 2, 3, 4, 5]).unwrap();
        let ary = list.into_array();
        assert_eq!(ary, vec![1, 2, 3, 4, 5])
    }
}
