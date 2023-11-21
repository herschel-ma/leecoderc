pub fn reverse(x: i32) -> i32 {
    let is_minus = x < 0;
    match x
        .abs()
        .to_string()
        .chars()
        .rev()
        .collect::<String>()
        .parse::<i32>()
    {
        Ok(x) => x * (if is_minus { -1 } else { 1 }),
        Err(_) => 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case1() {
        let x = 123;
        let res = 321;
        assert_eq!(reverse(x), res);
    }
    #[test]
    fn test_case2() {
        let x = -123;
        let res = -321;
        assert_eq!(reverse(x), res)
    }

    #[test]
    fn test_case3() {
        let x = 120;
        let res = 21;
        assert_eq!(reverse(x), res)
    }
}

