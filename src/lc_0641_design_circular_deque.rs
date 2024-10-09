use std::collections::VecDeque;

#[allow(dead_code)]
struct MyCircularDeque {
    store: VecDeque<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
#[allow(dead_code)]
impl MyCircularDeque {
    fn new(k: i32) -> Self {
        MyCircularDeque {
            store: VecDeque::with_capacity(k as usize),
        }
    }

    fn insert_front(&mut self, value: i32) -> bool {
        if self.store.len() >= self.store.capacity() {
            return false;
        }
        self.store.push_front(value);
        true
    }

    fn insert_last(&mut self, value: i32) -> bool {
        if self.store.len() >= self.store.capacity() {
            return false;
        }
        self.store.push_back(value);
        true
    }

    fn delete_front(&mut self) -> bool {
        self.store.pop_front().is_some()
    }

    fn delete_last(&mut self) -> bool {
        self.store.pop_back().is_some()
    }

    fn get_front(&self) -> i32 {
        *self.store.front().unwrap_or(&-1)
    }

    fn get_rear(&self) -> i32 {
        *self.store.back().unwrap_or(&-1)
    }

    fn is_empty(&self) -> bool {
        self.store.is_empty()
    }

    fn is_full(&self) -> bool {
        self.store.len() == self.store.capacity()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_circular_dequeue() {
        let mut my_circular_deque = MyCircularDeque::new(3);
        let mut output = Vec::new();
        let r1 = my_circular_deque.insert_last(1);
        output.push(r1);
        let r2 = my_circular_deque.insert_last(2);
        output.push(r2);
        let r3 = my_circular_deque.insert_front(3);
        output.push(r3);
        let r4 = my_circular_deque.insert_front(4);
        output.push(r4);
        let r5 = my_circular_deque.get_rear();
        assert_eq!(2, r5);
        let r6 = my_circular_deque.is_full();
        output.push(r6);
        let r7 = my_circular_deque.delete_last();
        output.push(r7);
        let r8 = my_circular_deque.insert_front(4);
        output.push(r8);
        let r9 = my_circular_deque.get_front();
        assert_eq!(4, r9);
        assert_eq!(vec![true, true, true, false, true, true, true], output);
    }
}
