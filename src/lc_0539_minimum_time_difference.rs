use crate::Solution;

impl Solution {
    pub fn find_min_difference(time_points: Vec<String>) -> i32 {
        let mut time_points: Vec<i32> = time_points
            .iter()
            .map(|t| t[..2].parse::<i32>().unwrap() * 60 + t[3..].parse::<i32>().unwrap())
            .collect();
        time_points.sort();
        let minutes = time_points.windows(2).map(|w| w[1] - w[0]).min().unwrap();
        minutes.min(24 * 60 + time_points.first().unwrap() - time_points.last().unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let time_points = vec!["23:59".to_string(), "00:00".to_string()];
        assert_eq!(Solution::find_min_difference(time_points), 1);
    }

    #[test]
    fn test_case_2() {
        let time_points = vec![
            "00:00".to_string(),
            "23:59".to_string(),
            "00:00".to_string(),
        ];
        assert_eq!(Solution::find_min_difference(time_points), 0);
    }
}
