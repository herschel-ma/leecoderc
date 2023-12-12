pub fn is_match(s: String, p: String) -> bool {
    let (m, n) = (s.len(), p.len());
    let sb = s.chars().collect::<Vec<char>>();
    let pb = p.chars().collect::<Vec<char>>();

    let mut dp = vec![vec![false; n + 1]; m + 1];
    dp[0][0] = true; // because empty pattern can match empty string
                     // in case 2, position of * should match char
                     //              *
                     //  [[true, false],
                     //a  [false, false],
                     //a  [false, false]]
    for j in 1..=n {
        if pb[j - 1] == '*' {
            dp[0][j] = dp[0][j - 1];
        }
    }

    for i in 1..=m {
        for j in 1..=n {
            if pb[j - 1] == '?' || sb[i - 1] == pb[j - 1] {
                dp[i][j] = dp[i - 1][j - 1];
            } else if pb[j - 1] == '*' {
                dp[i][j] = dp[i][j - 1] || dp[i - 1][j];
            }
        }
    }
    dp[m][n]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let s = String::from("aa");
        let p = String::from("a");
        let out = false;
        assert_eq!(is_match(s, p), out);
    }

    #[test]
    fn test_case_2() {
        let s = String::from("aa");
        let p = String::from("*");
        let out = true;
        assert_eq!(is_match(s, p), out);
    }
    #[test]
    fn test_case_3() {
        let s = String::from("cb");
        let p = String::from("?a");
        let out = false;
        assert_eq!(is_match(s, p), out);
    }
    #[test]
    fn test_wrong_answer() {
        let s = "adceb".to_string();
        let p = "*a*b".to_string();
        assert!(is_match(s, p));
    }

    #[test]
    fn test_string_index() {
        let s: String = String::from("hello");
        let c = 'h';
        assert_eq!(s.find(c), Some(0));
        assert_eq!(s.chars().collect::<Vec<char>>()[0], c);
    }
}
