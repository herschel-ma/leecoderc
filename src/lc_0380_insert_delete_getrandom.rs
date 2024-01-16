use rand::prelude::*;

#[derive(Debug, Default)]
struct RandomizedSet {
    array: Vec<i32>,
}

/// Implement the `RandomizedSet` class:
///     - `RandomizedSet()` Initializes the `RandomizedSet` object.
///     - `bool insert(int val)` Inserts an item `val` into the set if not present.
/// Returns `true` if the item was not present, `false` otherwise.
///     - `bool remore(int val)` Removes an item `val` from the set if present.
/// Returns `true` if the item was present, `false` otherwise.
///     - `int getRandom()` Returns a random element from the current set of elements
/// (it's guaranteed that at least one element exists when this method is called).
/// Each element must have the `same probaility` of being returned.
///
/// You must implement the functions of class such that each function works in `avarage O(1)` time
#[allow(unused)]
impl RandomizedSet {
    fn new() -> Self {
        Default::default()
    }

    fn insert(&mut self, val: i32) -> bool {
        if self.array.contains(&val) {
            false
        } else {
            self.array.push(val);
            true
        }
    }

    fn remove(&mut self, val: i32) -> bool {
        if let Some(index) = self.array.iter().position(|&x| x == val) {
            self.array.remove(index);
            return true;
        }
        false
    }

    fn get_random(&self) -> i32 {
        let mut rng = rand::thread_rng();
        *self.array.choose(&mut rng).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    enum Operation {
        RandomizedSet,
        Insert,
        Remove,
        GetRandom,
    }

    #[test]
    fn test_case_1() {
        let operations = vec![
            Operation::RandomizedSet,
            Operation::Insert,
            Operation::Remove,
            Operation::Insert,
            Operation::GetRandom,
            Operation::Remove,
            Operation::Insert,
            Operation::GetRandom,
        ];
        let vals = vec![
            vec![],
            vec![1],
            vec![2],
            vec![2],
            vec![],
            vec![1],
            vec![2],
            vec![],
        ];

        let output: Vec<Option<serde_json::Value>> = vec![
            None,
            Some(json!(true)),
            Some(json!(false)),
            Some(json!(true)),
            Some(json!(2)),
            Some(json!(true)),
            Some(json!(false)),
            Some(json!(2)),
        ];

        let output2: Vec<Option<serde_json::Value>> = vec![
            None,
            Some(json!(true)),
            Some(json!(false)),
            Some(json!(true)),
            Some(json!(1)),
            Some(json!(true)),
            Some(json!(false)),
            Some(json!(2)),
        ];

        let mut res = operations
            .into_iter()
            .skip(1)
            .zip(vals.into_iter().skip(1))
            .fold(
                (vec![], RandomizedSet::new()),
                |mut acc, (en, val)| match en {
                    Operation::Insert => {
                        let ret: bool = acc.1.insert(val[0]);
                        acc.0.push(Some(json!(ret)));
                        acc
                    }
                    Operation::Remove => {
                        let ret: bool = acc.1.remove(val[0]);
                        acc.0.push(Some(json!(ret)));
                        acc
                    }
                    Operation::GetRandom => {
                        let ret: i32 = acc.1.get_random();
                        acc.0.push(Some(json!(ret)));
                        acc
                    }
                    _ => acc,
                },
            );

        res.0.insert(0, None);
        assert!([output, output2].into_iter().any(|o| *o == res.0));
    }
}

