pub fn con_construct(ransom_note: String, magazine: String) -> bool {
    use std::collections::HashMap;
    let mut hm = HashMap::new();

    for ch in magazine.chars() {
        *hm.entry(ch).or_insert(0) += 1;
    }

    for ch in ransom_note.chars() {
        let e = hm.entry(ch).or_insert(0);
        if *e == 0 {
            return false;
        }
        *e -= 1
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        assert!(!con_construct("a".to_string(), "b".to_string()))
    }

    #[test]
    fn ex2() {
        assert!(!con_construct("aa".to_string(), "ab".to_string()))
    }
    #[test]
    fn ex3() {
        assert!(con_construct("aa".to_string(), "aab".to_string()))
    }
}
