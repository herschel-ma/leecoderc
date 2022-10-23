pub fn is_palindrome(s: String) -> bool {
    let forward: String = s
        .to_lowercase()
        .chars()
        .filter(|ch| ch.is_alphanumeric())
        .collect();

    let backward: String = forward.chars().rev().collect();


    forward == backward
}


mod tests {
    use crate::is_palindrome;

    #[test]
    fn ex1() {
        let s = String::from("A man, a plan, a canal: Panama");
        assert!(is_palindrome(s));
    }

    #[test]
    fn ex2() {
        let s = String::from("race a car");
        assert!(!is_palindrome(s));
    }

    #[test]
    fn ex3() {
        let s = String::from(" ");
        assert!(is_palindrome(s));
    }
}