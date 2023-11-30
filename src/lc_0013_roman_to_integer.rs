use std::collections::HashMap;

pub fn roman_to_int(s: String) -> i32 {
    let mut ans = 0;
    let length = s.len();
    let chas = s.chars().collect::<Vec<char>>();
    let mut map: HashMap<char, i32> = HashMap::new();

    map.insert('I', 1);
    map.insert('V', 5);
    map.insert('X', 10);
    map.insert('L', 50);
    map.insert('C', 100);
    map.insert('D', 500);
    map.insert('M', 1000);

    for i in 0..length - 1 {
        let cur = map.get(&chas[i]).unwrap();
        let next = map.get(&chas[i + 1]).unwrap();
        if cur < next {
            ans -= cur;
        } else {
            ans += cur;
        }
    }
    ans += map.get(chas.get(length - 1).unwrap()).unwrap();
    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case1() {
        let s = "III".to_string();
        let output = 3;
        assert_eq!(roman_to_int(s), output)
    }

    #[test]
    fn test_case2() {
        let s = "LVIII".to_string();
        let output = 58;
        assert_eq!(roman_to_int(s), output)
    }

    #[test]
    fn test_case3() {
        let s = "MCMXCIV".to_string();
        let output = 1994;
        assert_eq!(roman_to_int(s), output)
    }
}
