pub fn int_to_roman(num: i32) -> String {
    let cs = vec![
        "M", "CM", "D", "CD", "C", "XC", "L", "XL", "X", "IX", "V", "IV", "I",
    ];
    let cs = cs.into_iter().map(|x| x.to_string());
    let vs = vec![1000, 900, 500, 400, 100, 90, 50, 40, 10, 9, 5, 4, 1];
    let mut ans: Vec<String> = Vec::new();
    let mut num = num;
    for (c, v) in cs.into_iter().zip(vs.into_iter()) {
        while num >= v {
            ans.push(c.clone());
            num -= v;
        }
    }
    ans.iter().map(|c| c.chars()).flatten().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case1() {
        let num = 3;
        let output = "III".to_string();
        assert_eq!(int_to_roman(num), output);
    }

    #[test]
    fn test_case2() {
        let num = 58;
        let output = "LVIII".to_string();
        assert_eq!(int_to_roman(num), output);
    }
}

