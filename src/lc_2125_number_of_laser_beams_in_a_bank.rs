use crate::Solution;

impl Solution {
    pub fn number_of_beams(bank: Vec<String>) -> i32 {
        bank.into_iter()
            .map(|s| s.chars().filter(|&c| c == '1').count() as i32)
            .filter(|&x| x > 0)
            .collect::<Vec<i32>>()
            .windows(2)
            .map(|x| x[0] * x[1])
            .collect::<Vec<i32>>()
            .into_iter()
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let bank = vec![
            "011001".into(),
            "000000".into(),
            "010100".into(),
            "001000".into(),
        ];
        assert_eq!(Solution::number_of_beams(bank), 8);
    }

    #[test]
    fn test_case_2() {
        let bank = vec!["000".into(), "111".into(), "000".into()];
        assert_eq!(Solution::number_of_beams(bank), 0);
    }
}

