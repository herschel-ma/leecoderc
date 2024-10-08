use std::collections::BTreeMap;

#[allow(dead_code)]
#[derive(Default)]
struct MyCalendar {
    bt: BTreeMap<i32, i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
#[allow(dead_code)]
impl MyCalendar {
    fn new() -> Self {
        Default::default()
    }

    #[allow(dead_code)]
    fn book(&mut self, start: i32, end: i32) -> bool {
        if let Some((&_, &e)) = self.bt.range(..end).next_back() {
            if start < e {
                return false;
            }
        }
        self.bt.insert(start, end);
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mycalendar() {
        let mut c = MyCalendar::new();
        let pairs = [[10, 20], [15, 25], [20, 30]];
        assert_eq!(
            pairs.iter().fold(vec![], |mut acc, x| {
                acc.push(c.book(x[0], x[1]));
                acc
            }),
            vec![true, false, true]
        )
    }
}
