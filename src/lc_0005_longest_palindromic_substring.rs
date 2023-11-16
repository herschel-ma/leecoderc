pub fn longest_palindrome_0005(s: String) -> String {
    let (n, mut ans) = (s.len(), &s[..1]);
    let mut dp = vec![vec![false; n]; n];
    let data = s.chars().collect::<Vec<char>>();

    for end in 1..n {
        for start in 0..=end {
            if data[start] == data[end] {
                dp[start][end] = end - start < 2 || dp[start + 1][end - 1];
                if dp[start][end] && end - start + 1 > ans.len() {
                    ans = &s[start..=end]
                }
            }
        }
    }
    ans.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case1() {
        let s = String::from("babad");
        let output = String::from("bab");
        assert_eq!(longest_palindrome_0005(s), output);
    }

    #[test]
    fn test_case2() {
        let s = String::from("cbbd");
        let output = String::from("bb");
        assert_eq!(longest_palindrome_0005(s), output);
    }
}
