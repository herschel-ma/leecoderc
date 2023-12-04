pub fn str_str(haystack: String, needle: String) -> i32 {
    let n = haystack.len();
    let m = needle.len();
    if n < m {
        return -1;
    }
    for i in 0..n - m + 1 {
        if haystack[i..i + m] == needle {
            return i as i32;
        }
    }
    -1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case1() {
        let haystack = String::from("sadbutsad");
        let needle = String::from("sad");
        let output = 0;
        assert_eq!(str_str(haystack, needle), output)
    }

    #[test]
    fn test_case2() {
        let haystack = String::from("leetcode");
        let needle = String::from("leeto");
        let output = -1;
        assert_eq!(str_str(haystack, needle), output);
    }
}

