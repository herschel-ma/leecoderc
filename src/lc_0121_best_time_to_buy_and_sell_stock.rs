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

mod tests {
    #[test]
    fn ex1() {
        use crate::max_profit;
        let prices = vec![7, 1, 5, 3, 6, 4];
        let expected = 5;
        assert_eq!(max_profit(prices), expected);
    }

    #[test]
    fn ex2() {
        use crate::max_profit;
        let prices = vec![7, 6, 4, 3, 1];
        let expected = 0;
        assert_eq!(max_profit(prices), expected);
    }
}
