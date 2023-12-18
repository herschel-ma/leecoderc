use crate::Solution;

impl Solution {
    pub fn full_justify(words: Vec<String>, max_width: i32) -> Vec<String> {
        let mut line = Vec::new();
        let mut ans = Vec::new();
        let mut width = 0;

        for w in &words {
            // If cur word length plus cur line words cnt - 1 (text need a space between two word)
            // plus previous width > max_width:
            // we can not put cur word here, and need to calculate where to put space and
            // generate the res line.
            if line.len() + w.len() + width > max_width as usize {
                for i in 0..max_width as usize - width {
                    let n = if line.len() > 1 { line.len() - 1 } else { 1 };
                    line[i % n] = format!("{} ", line[i % n]);
                }
                ans.push(line.join(""));
                line.clear();
                width = 0;
            }
            line.push(w.clone());
            width += w.len();
        }
        let last_line = line.join(" ");
        ans.push(format!("{:<width$}", last_line, width = max_width as usize));
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let words = vec![
            "This".to_string(),
            "is".to_string(),
            "an".to_string(),
            "example".to_string(),
            "of".to_string(),
            "text".to_string(),
            "justification.".to_string(),
        ];
        let max_width = 16;
        let output = vec![
            "This    is    an".to_string(),
            "example  of text".to_string(),
            "justification.  ".to_string(),
        ];
        assert_eq!(Solution::full_justify(words, max_width), output);
    }

    #[test]
    fn test_case_2() {
        let words = vec![
            "What".to_string(),
            "must".to_string(),
            "be".to_string(),
            "acknowledgment".to_string(),
            "shall".to_string(),
            "be".to_string(),
        ];
        let max_width = 16;
        let output = vec![
            "What   must   be".to_string(),
            "acknowledgment  ".to_string(),
            "shall be        ".to_string(),
        ];
        assert_eq!(Solution::full_justify(words, max_width), output);
    }

    #[test]
    fn test_case_3() {
        let words = vec![
            "Science".to_string(),
            "is".to_string(),
            "what".to_string(),
            "we".to_string(),
            "understand".to_string(),
            "well".to_string(),
            "enough".to_string(),
            "to".to_string(),
            "explain".to_string(),
            "to".to_string(),
            "a".to_string(),
            "computer.".to_string(),
            "Art".to_string(),
            "is".to_string(),
            "everything".to_string(),
            "else".to_string(),
            "we".to_string(),
            "do".to_string(),
        ];
        let max_width = 20;
        let output = vec![
            "Science  is  what we".to_string(),
            "understand      well".to_string(),
            "enough to explain to".to_string(),
            "a  computer.  Art is".to_string(),
            "everything  else  we".to_string(),
            "do                  ".to_string(),
        ];
        assert_eq!(Solution::full_justify(words, max_width), output);
    }
}
