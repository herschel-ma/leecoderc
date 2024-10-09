use std::collections::HashMap;

#[derive(Default)]
#[allow(dead_code)]
struct AllOne {
    m: HashMap<String, i32>,
    o: i32,
    res: String,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
#[allow(dead_code)]
impl AllOne {
    fn new() -> Self {
        Default::default()
    }

    fn inc(&mut self, key: String) {
        self.o = 0;
        *self.m.entry(key).or_insert(0) += 1;
    }

    fn dec(&mut self, key: String) {
        self.o = 0;
        if let Some(v) = self.m.get_mut(&key) {
            if *v > 1 {
                *v -= 1;
            } else {
                self.m.remove(&key);
            }
        }
    }

    fn get_max_key(&mut self) -> String {
        if self.o == 1 {
            return self.res.clone();
        }
        self.o = 1;
        if !self.m.is_empty() {
            let mut max_key = String::new();
            let mut max_value = i32::MIN;
            for (k, &v) in &self.m {
                if v > max_value {
                    max_value = v;
                    max_key = k.clone();
                }
            }
            self.res = max_key.clone();
            return max_key;
        }
        self.res.clear();
        String::new()
    }

    fn get_min_key(&mut self) -> String {
        if self.o == 2 {
            return self.res.clone();
        }
        let mut min_key = String::new();
        let mut min_value = i32::MAX;
        self.o = 2;
        if !self.m.is_empty() {
            for (k, &v) in &self.m {
                if v < min_value {
                    min_value = v;
                    min_key = k.clone();
                }
            }
            self.res = min_key.clone();
            return min_key;
        }
        self.res.clear();
        String::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case() {
        let mut allone = AllOne::new();
        allone.inc("hello".to_string());
        allone.inc("hello".to_string());
        assert_eq!("hello".to_string(), allone.get_max_key());
        allone.inc("leet".to_string());
        assert_eq!("hello".to_string(), allone.get_max_key());
        assert_eq!("leet".to_string(), allone.get_min_key());
    }
}
