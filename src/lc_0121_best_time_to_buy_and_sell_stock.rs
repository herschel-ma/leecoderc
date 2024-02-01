use crate::Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        if prices.len() < 2 {
            return 0;
        }

        let mut min_prices = prices[0];
        let mut max_profit = prices[1] - prices[0];

        for price in prices.into_iter().skip(1) {
            min_prices = min_prices.min(price);
            max_profit = max_profit.max(price - min_prices);
        }
        max_profit
    }
}

#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let prices = vec![7, 1, 5, 3, 6, 4];
        let expected = 5;
        assert_eq!(Solution::max_profit(prices), expected);
    }

    #[test]
    fn test_case_2() {
        let prices = vec![7, 6, 4, 3, 1];
        let expected = 0;
        assert_eq!(Solution::max_profit(prices), expected);
    }
}
