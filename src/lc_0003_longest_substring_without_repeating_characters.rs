use std::collections::HashMap;
use std::collections::HashSet;

pub fn length_of_longest_substring_v1(s: String) -> i32 {
    let array = s.chars().collect::<Vec<char>>();

    let mut hs = HashSet::new();

    let mut start = 0;
    let mut end = 0;
    let mut max_len = 0;
    while end < array.len() {
        if !hs.insert(array[end]) {
            while array[start] != array[end] {
                hs.remove(&array[start]);
                start += 1;
            }
            start += 1;
        }
        end += 1;
        max_len = max_len.max(hs.len());
    }

    max_len as i32
}

// 相当于取一个滑动窗口，求滑动窗口的最大长度
pub fn length_of_longest_substring(s: String) -> i32 {
    let mut map: HashMap<char, usize> = HashMap::new();
    let mut i = 0;
    let mut res = 0;
    for (j, c) in s.chars().enumerate() {
        if let Some(prev_pos) = map.get(&c) {
            i = i.max(prev_pos + 1);
        }
        res = res.max(j - i + 1);
        map.insert(c, j);
    }
    res as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(length_of_longest_substring("abcabcbb".to_string()), 3);
    }

    #[test]
    fn ex2() {
        assert_eq!(length_of_longest_substring("bbbbb".to_string()), 1);
    }

    #[test]
    fn ex3() {
        assert_eq!(length_of_longest_substring("pwwkew".to_string()), 3);
    }
}
