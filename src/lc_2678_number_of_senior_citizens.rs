use crate::Solution;

impl Solution {
    pub fn count_seniors(details: Vec<String>) -> i32 {
        details
            .iter()
            .filter_map(|x| {
                x[11..=12].parse::<i32>().ok()
            })
            .filter(|age| *age > 60)
            .count() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let details = vec![
            "7868190130M7522".to_string(),
            "5303914400F9211".to_string(),
            "9273338290F4010".to_string(),
        ];
        assert_eq!(Solution::count_seniors(details), 2);
    }

    #[test]
    fn test_case_2() {
        let details = vec!["1313579440F2036".to_string(), "2921522980M5644".to_string()];
        assert_eq!(Solution::count_seniors(details), 0);
    }
}
