use crate::{ListNode, Solution};
impl Solution {
    pub fn insert_greatest_common_divisors(
        mut head: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        fn gcd(a: i32, b: i32) -> i32 {
            if b == 0 {
                a
            } else {
                gcd(b, a % b)
            }
        }

        let mut current = &mut head;
        while let Some(n) = current {
            if n.next.is_some() {
                let val = gcd(n.val, n.next.as_ref().unwrap().val);
                let mut new = Box::new(ListNode::new(val));
                new.next = n.next.take();

                n.next = Some(new);
                current = &mut n.next.as_mut().unwrap().next;
            } else {
                break;
            }
        }
        head
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let head = ListNode::from_vec(&[18, 6, 10, 3]);
        let output = ListNode::from_vec(&[18, 6, 6, 2, 10, 1, 3]);
        assert_eq!(Solution::insert_greatest_common_divisors(head), output);
    }

    #[test]
    fn test_case_2() {
        let head = ListNode::from_vec(&[7]);
        let output = ListNode::from_vec(&[7]);
        assert_eq!(Solution::insert_greatest_common_divisors(head), output);
    }

    #[test]
    fn test_case_3() {
        let head = ListNode::from_vec(&[12, 7]);
        let output = ListNode::from_vec(&[12, 1, 7]);
        assert_eq!(Solution::insert_greatest_common_divisors(head), output);
    }
}
