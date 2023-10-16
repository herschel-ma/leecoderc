use std::collections::HashSet;

pub fn contains_duplicate(nums: Vec<i32>) -> bool {
    let mut hs = HashSet::new();
    for num in nums {
        if !hs.insert(num) {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        assert!(contains_duplicate(vec![1, 2, 3, 1]))
    }

    #[test]
    fn ex2() {
        assert!(!contains_duplicate(vec![1, 2, 3, 4]))
    }

    #[test]
    fn ex3() {
        assert!(contains_duplicate(vec![1, 1, 1, 3, 3, 4, 3, 2, 4, 2]))
    }
}
