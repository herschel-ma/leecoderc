// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  pub fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
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
}


mod tests {
    use super::*;

    #[test]
    pub fn creat_list() {
        let mut list = ListNode::from_vec(&vec![1,2,3,4,5]);

        let mut n = 1;
        while let Some(entry) = list {
            assert_eq!(entry.val, n);
            n += 1;
            list = entry.next
        }
    }
}