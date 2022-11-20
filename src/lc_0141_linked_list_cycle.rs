// no rust solutiion in leetcode official website, use python solution.

// class Solution:
//     def hasCycle(self, head: Optional[ListNode]) -> bool:
//         try:
//             slow = head
//             fast = head.next
//
//             while slow != fast:
//                 slow = slow.next
//                 fast = fast.next.next
//
//             return True
//         except Exception:
//             return False
