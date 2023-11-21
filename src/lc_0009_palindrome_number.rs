pub fn is_palidrome(x: i32) -> bool {
    if x < 0 {
        return false;
    }
    let cs = x.to_string().chars().collect::<Vec<char>>();
    let sz = cs.len();
    for i in 0..sz {
        if cs[i] != cs[sz - i - 1] {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case1() {
        let x = 121;
        assert!(is_palidrome(x))
    }

    #[test]
    fn test_case2() {
        let x = -121;
        assert!(!is_palidrome(x),)
    }

    #[test]
    fn test_case3() {
        let x = 10;
        assert!(!is_palidrome(x))
    }
}

