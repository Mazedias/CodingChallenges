// Problem 121
struct Solution;

#[allow(dead_code)]
impl Solution {

    pub fn max_profit (prices: Vec<i32>) -> i32 {
        if prices.len() < 2 {
            return 0;
        }

        let mut buy = prices[0];
        let mut sell = 0;

        for &price in &prices[1..] {
            if price < buy {
                buy = price;
            } else if price - buy > sell {
                sell = price - buy;
            }
        }

        sell
    }

    /**
     * First solution but can be done better
     */
    pub fn max_profit_b (prices: Vec<i32>) -> i32 {
        if prices.len() == 0 {
            return 0;
        }

        let mut low = prices[0];
        let mut high = prices[0];
        let mut diff = 0;
        
        for price in prices {
            if price > high {
                high = price;
            } else if price < low {
                low = price;
                high = price;
            }

            if (high - low) > diff {
                diff = high - low;
            }
        }

        diff
    }
}

#[test]
pub fn test_max_profit () {
    let a = vec![7, 1, 5, 3, 6, 4];
    let b = vec![7, 6, 4, 3, 1];
    let c = vec![7, 7, 4, 20, 1, 20];


    assert_eq!(Solution::max_profit(a), 5);
    assert_eq!(Solution::max_profit(b), 0);
    assert_eq!(Solution::max_profit(c), 19);
}