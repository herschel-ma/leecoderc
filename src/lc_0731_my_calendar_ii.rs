#[derive(Default)]
struct MyCalendarTwo {
    pub s: Vec<(i32, i32)>,
    pub d: Vec<(i32, i32)>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
#[allow(dead_code)]
impl MyCalendarTwo {
    fn new() -> Self {
        Default::default()
    }

    fn book(&mut self, start: i32, end: i32) -> bool {
        for (s, e) in &self.d {
            if start >= *e || end <= *s {
                continue;
            }
            return false;
        }

        for (s, e) in self.s.clone() {
            if start >= e || end <= s {
                continue;
            }
            let new_start = if start > s { start } else { s };
            let new_end = if end < e { end } else { e };
            self.d.push((new_start, new_end));
        }

        self.s.push((start, end));
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mycalendar() {
        let mut c = MyCalendarTwo::new();
        let pairs = [[10, 20], [50, 60], [10, 40], [5, 15], [5, 10], [25, 55]];
        assert_eq!(
            pairs.iter().fold(vec![], |mut acc, x| {
                acc.push(c.book(x[0], x[1]));
                acc
            }),
            vec![true, true, true, false, true, true]
        )
    }
}
