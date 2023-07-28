pub fn maximum_profit(stock_prices: &[usize]) -> usize {
    let mut max_profit = 0;

    for (i, &price) in stock_prices.iter().enumerate() {
        let profit = stock_prices.iter().skip(i).max().map(|max| max - price);
        if profit.is_some_and(|prof| prof > max_profit) {
            max_profit = profit.unwrap();
        }
    }

    max_profit
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(maximum_profit(&[7, 1, 5, 3, 6, 4]), 5);
        assert_eq!(maximum_profit(&[]), 0);
        assert_eq!(maximum_profit(&[5, 4, 3, 2, 1]), 0);
    }
}
