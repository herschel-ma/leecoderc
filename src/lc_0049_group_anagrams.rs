use std::collections::HashMap;

pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    let mut map: HashMap<String, Vec<String>> = HashMap::new();
    for s in strs {
        let key: String = {
            let mut arr = s.chars().collect::<Vec<char>>();
            arr.sort();
            arr.into_iter().collect()
        };
        let val = map.entry(key).or_default();
        val.push(s);
    }
    map.into_values().collect::<Vec<Vec<String>>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let strs = vec![
            "eat".to_string(),
            "tea".to_string(),
            "tan".to_string(),
            "ate".to_string(),
            "nat".to_string(),
            "bat".to_string(),
        ];
        let output = vec![
            vec!["bat".to_string()],
            vec!["nat".to_string(), "tan".to_string()],
            vec!["ate".to_string(), "eat".to_string(), "tea".to_string()],
        ];
        assert_eq!(group_anagrams(strs).len(), output.len());
    }

    #[test]
    fn test_case_2() {
        let strs = vec!["".to_string()];
        let output = vec![vec!["".to_string()]];
        assert_eq!(group_anagrams(strs).len(), output.len());
    }

    #[test]
    fn test_case_3() {
        let strs = vec!["a".to_string()];
        let output = vec![vec!["a".to_string()]];
        assert_eq!(group_anagrams(strs).len(), output.len());
    }
}
