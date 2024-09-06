use crate::{ListNode, Solution};

impl Solution {
    pub fn modified_list(nums: Vec<i32>, head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut exclude_set = std::collections::HashSet::new();
        for n in nums.iter() {
            exclude_set.insert(n);
        }

        let mut dummy = Box::new(ListNode::new(0));
        dummy.next = head;
        let mut curr = &mut dummy;

        while let Some(next) = curr.next.take() {
            if exclude_set.contains(&next.val) {
                curr.next = next.next;
            } else {
                curr.next = Some(next);
                curr = curr.next.as_mut().unwrap();
            }
        }

        dummy.next
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let nums = vec![1, 2, 3];
        let head = ListNode::from_vec(&[1, 2, 3, 4, 5]);
        let output = ListNode::from_vec(&[4, 5]);
        assert_eq!(Solution::modified_list(nums, head), output)
    }

    #[test]
    fn test_case_2() {
        let nums = vec![1];
        let head = ListNode::from_vec(&[1, 2, 1, 2, 1, 2]);
        let output = ListNode::from_vec(&[2, 2, 2]);
        assert_eq!(Solution::modified_list(nums, head), output)
    }

    #[test]
    fn test_case_3() {
        let nums = vec![5];
        let head = ListNode::from_vec(&[1, 2, 3, 4]);
        let output = ListNode::from_vec(&[1, 2, 3, 4]);
        assert_eq!(Solution::modified_list(nums, head), output)
    }
}
