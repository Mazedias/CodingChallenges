// Problem 2070
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn maximum_beauty (mut items: Vec<Vec<i32>>, queries: Vec<i32>) -> Vec<i32> {
        let mut temp: Vec<i32> = vec![0; queries.len()];
        
        items.sort_by(|a, b| {
            a[0].cmp(&b[0])
        });

        let mut indexed_queries: Vec<(usize, i32)> = (0..queries.len()).map(|i| (i, queries[i].clone())).collect();
        indexed_queries.sort_by(|(_, a), (_, b)| {
            a.cmp(b)
        });

        let mut pos = 0;
        let mut max_beauty = 0;
        for i in 0..indexed_queries.len() {
            if i > 0 {
                temp[indexed_queries[i].0] = max_beauty;
            }

            while pos < items.len() && indexed_queries[i].1 >= items[pos][0] {
                if items[pos][1] > temp[indexed_queries[i].0] {
                    temp[indexed_queries[i].0] = items[pos][1];
                    max_beauty = items[pos][1];
                }
                pos += 1;
            }
        }

        temp
    }
}

#[test]
pub fn test_maximum_beauty () {
    let items1: Vec<Vec<i32>> = vec![vec![1, 2], vec![3, 2], vec![2, 4], vec![5, 6], vec![3, 5]];
    let queries1: Vec<i32> = vec![1,2,3,4,5,6];

    let items2: Vec<Vec<i32>> = vec![vec![1, 2], vec![1, 2], vec![1, 3], vec![1, 4]];
    let queries2: Vec<i32> = vec![1];

    let items3: Vec<Vec<i32>> = vec![vec![10, 10000]];
    let queries3: Vec<i32> = vec![5];

    let items4: Vec<Vec<i32>> = vec![vec![193, 732], vec![781, 962], vec![864, 954], vec![749, 672], vec![136, 746], vec![478, 548], vec![640, 908], vec![210, 799], vec![567, 715], vec![914,388], vec![487,853], vec![533,554], vec![247,919], vec![958,150], vec![193,523], vec![176,656], vec![395,469], vec![763,821], vec![542,946], vec![701,676]];
    let queries4: Vec<i32> = vec![885,1445,1580,1309,205,1788,1214,1404,572,1170,989,265,153,151,1479,1180,875,276,1584];

    assert_eq!(Solution::maximum_beauty(items4, queries4), vec![962,962,962,962,746,962,962,962,946,962,962,919,746,746,962,962,962,919,962]);
    assert_eq!(Solution::maximum_beauty(items1, queries1), vec![2,4,5,5,6,6]);
    assert_eq!(Solution::maximum_beauty(items2, queries2), vec![4]);
    assert_eq!(Solution::maximum_beauty(items3, queries3), vec![0]);
}