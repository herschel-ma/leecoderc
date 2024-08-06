use crate::Solution;

impl Solution {
    pub fn minimum_pushes(word: String) -> i32 {
        let digits = (2..=9).count();
        let mut counts = word
            .chars()
            .fold(std::collections::HashMap::new(), |mut acc, e| {
                acc.entry(e).and_modify(|n| *n += 1).or_insert(1);
                acc
            })
            .into_values()
            .collect::<Vec<_>>();
        counts.sort_by(|x, y| y.cmp(x));
        counts
            .iter()
            .step_by(digits)
            .enumerate()
            .map(|(i, _)| (i + 1) * counts.iter().skip(i * digits).take(digits).sum::<usize>())
            .sum::<usize>() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let word = "abcde".to_string();
        assert_eq!(Solution::minimum_pushes(word), 5);
    }

    #[test]
    fn test_case_2() {
        let word = "xyzxyzxyzxyz".to_string();
        assert_eq!(Solution::minimum_pushes(word), 12);
    }
    #[test]
    fn test_case_3() {
        let word = "aabbccddeeffgghhiiiiii".to_string();
        assert_eq!(Solution::minimum_pushes(word), 24);
    }
}
