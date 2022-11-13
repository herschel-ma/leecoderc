pub fn longest_palindrome(s: String) -> i32 {
    let mut table = std::collections::HashMap::new();
    for ch in s.chars() {
        *table.entry(ch).or_insert(0) += 1
    }
    let mut max_len = 0;
    let mut has_odd = false;
    for val in table.values() {
        if val % 2 == 0 {
            max_len += val
        } else {
            max_len += val - 1;
            has_odd = true;
        }
    }
    max_len + has_odd as i32
}

#[cfg(test)]
mod tests {
    use crate::longest_palindrome;

    #[test]
    fn ex1() {
        assert_eq!(longest_palindrome("abccccdd".to_string()), 7)
    }

    #[test]
    fn ex2() {
        assert_eq!(longest_palindrome("a".to_string()), 1)
    }
}
