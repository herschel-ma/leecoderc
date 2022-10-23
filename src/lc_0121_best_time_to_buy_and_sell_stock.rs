pub fn max_profit(prices: Vec<i32>) -> i32 {
    if prices.len() < 2 {
        return 0;
    }
    
    let mut min_prices = prices[0];
    let mut max_profit = prices[1] - prices[0];

    for i in 1..prices.len() {
        min_prices = min_prices.min(prices[i]);
        max_profit = max_profit.max(prices[i] - min_prices);
    }
    max_profit

}

mod tests {
    use crate::max_profit;

    #[test]
    fn ex1() {
        let prices = vec![7,1,5,3,6,4];
        let expected = 5;
        assert_eq!(max_profit(prices), expected);
    }

    #[test]
    fn ex2() {
        let prices = vec![7,6,4,3,1];
        let expected = 0;
        assert_eq!(max_profit(prices), expected);
    }
}