use std::cmp::Reverse;
use std::collections::BinaryHeap;

#[allow(dead_code)]
#[derive(Debug)]
struct KthLargest(BinaryHeap<Reverse<i32>>);

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
#[allow(dead_code)]
impl KthLargest {
    fn new(k: i32, nums: Vec<i32>) -> Self {
        let k = k as usize;
        let h: BinaryHeap<_> = nums
            .iter()
            .chain(std::iter::repeat(&i32::MIN))
            .take(k)
            .map(|&x| Reverse(x))
            .collect();

        let mut this = Self(h);
        for &x in nums.iter().skip(k) {
            this.add(x);
        }

        this
    }

    fn add(&mut self, val: i32) -> i32 {
        {
            let mut top = self.0.peek_mut().unwrap();
            if val > top.0 {
                top.0 = val;
            }
        }
        self.0.peek().unwrap().0
    }
}

/**
 * Your KthLargest object will be instantiated and called as such:
 * let obj = KthLargest::new(k, nums);
 * let ret_1: i32 = obj.add(val);
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case1() {
        let mut ans = vec![];
        let input = (
            vec![
                "KthLargest".to_string(),
                "add".to_string(),
                "add".to_string(),
                "add".to_string(),
                "add".to_string(),
                "add".to_string(),
            ],
            vec![
                vec![3, 4, 5, 8, 2],
                vec![3],
                vec![5],
                vec![10],
                vec![9],
                vec![4],
            ],
        );
        let mut i1 = input.0.into_iter();
        let mut i2 = input.1.into_iter();

        let in1 = i1.next().unwrap();
        let in2 = i2.next().unwrap();

        assert_eq!(in1, "KthLargest");
        let mut obj = KthLargest::new(in2[0], in2[1..].to_vec());

        for v in i2 {
            let ret = obj.add(*v.first().unwrap());
            ans.push(ret)
        }

        assert_eq!(ans, vec![4, 5, 5, 8, 8])
    }
}

