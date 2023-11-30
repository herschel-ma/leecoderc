/// [题解](https://leetcode.com/problems/letter-combinations-of-a-phone-number/solutions/3855474/100-backtracking-iterative-video-letter-combinations-of-a-phone-number/)
pub fn letter_combinations(digits: String) -> Vec<String> {
    if digits.is_empty() {
        return vec![];
    }
    let phone_map = vec!["abc", "def", "ghi", "jkl", "mno", "pqrs", "tuv", "wxyz"];
    let mut output = Vec::new();
    fn backtrack(
        combination: String,
        next_digits: &str,
        phone_map: &Vec<&str>,
        output: &mut Vec<String>,
    ) {
        if next_digits.is_empty() {
            output.push(combination);
        } else {
            let letters = phone_map[next_digits.chars().next().unwrap() as usize - '2' as usize];
            for letter in letters.chars() {
                let new_combination = combination.clone() + &letter.to_string();
                backtrack(new_combination, &next_digits[1..], phone_map, output);
            }
        }
    }
    backtrack(String::new(), &digits, &phone_map, &mut output);

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case1() {
        let digits = "23".to_string();
        let output = vec![
            "ad".to_string(),
            "ae".to_string(),
            "af".to_string(),
            "bd".to_string(),
            "be".to_string(),
            "bf".to_string(),
            "cd".to_string(),
            "ce".to_string(),
            "cf".to_string(),
        ];
        assert_eq!(letter_combinations(digits), output)
    }

    #[test]
    fn test_case2() {
        let digits = "2".to_string();
        let output = vec!["a".to_string(), "b".to_string(), "c".to_string()];
        assert_eq!(letter_combinations(digits), output)
    }
}
