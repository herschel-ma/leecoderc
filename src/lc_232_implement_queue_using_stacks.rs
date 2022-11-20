struct MyQueue {
    stack1: Vec<i32>,
    stack2: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyQueue {
    fn new() -> Self {
        Self {
            stack1: vec![],
            stack2: vec![],
        }
    }

    fn push(&mut self, x: i32) {
        while let Some(val) = self.stack1.pop() {
            self.stack2.push(val);
        }
        self.stack1.push(x);
        while let Some(val) = self.stack2.pop() {
            self.stack1.push(val);
        }
    }

    fn pop(&mut self) -> i32 {
        self.stack1.pop().unwrap()
    }

    fn peek(&self) -> i32 {
        *self.stack1.last().unwrap()
    }

    fn empty(&self) -> bool {
        self.stack1.is_empty()
    }
}

/**
 * Your MyQueue object will be instantiated and called as such:
 * let obj = MyQueue::new();
 * obj.push(x);
 * let ret_2: i32 = obj.pop();
 * let ret_3: i32 = obj.peek();
 * let ret_4: bool = obj.empty();
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let mut q = MyQueue::new();
        q.push(1);
        q.push(2);
        assert_eq!(q.peek(), 1);
        assert_eq!(q.pop(), 1);
        assert_eq!(q.peek(), 2);
        assert!(!q.empty());
        assert_eq!(q.pop(), 2);
        assert!(q.empty());
    }
}
