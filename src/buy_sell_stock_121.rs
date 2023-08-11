pub fn max_profit(prices: Vec<i32>) -> i32 {
    max_profit_naive(prices)
}

fn max_profit_naive(prices: Vec<i32>) -> i32 {
    let mut last_maximum: Option<&i32> = None;
    let mut profit = 0;

    for (i, price) in prices.iter().enumerate().rev() {
        if last_maximum.is_none() || Some(price) > last_maximum {
            last_maximum = Some(price);
            if let Some(min) = prices[..i].iter().min() {
                profit = profit.max(price - min);
            }
        }
    }

    profit.max(0)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example4() {
        do_test(vec![1, 10, 5, 8], 9);
    }

    #[test]
    fn example3() {
        do_test(vec![1, 3, 2, 10], 9);
    }

    #[test]
    fn example2() {
        do_test(vec![1], 0);
    }

    #[test]
    fn example1() {
        do_test(vec![], 0);
    }

    #[test]
    fn leet_example3() {
        do_test(vec![3, 3, 5, 0, 0, 3, 1, 4], 4);
    }

    #[test]
    fn leet_example2() {
        do_test(vec![7, 6, 4, 3, 1], 0);
    }

    #[test]
    fn leet_example1() {
        do_test(vec![7, 1, 5, 3, 6, 4], 5);
    }

    fn do_test(input: Vec<i32>, expected: i32) {
        assert_eq!(max_profit(input), expected);
    }
}
